mod data;
mod payloads;

use std::{
    borrow::Cow,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};

use anyhow::{anyhow, Result};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use iced::{subscription, Subscription};
use once_cell::sync::OnceCell;
use serde_json::Value;
use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot, Mutex, RwLock},
    time,
};
use tokio_tungstenite::{
    tungstenite::{
        protocol::{frame::coding::CloseCode, CloseFrame},
        Message,
    },
    MaybeTlsStream, WebSocketStream,
};
use tracing::{error, info, warn};

use crate::{
    api::gateway::{data::DispatchMessage, payloads::identify_payload},
    data::state::State,
};

use self::{
    data::{DispatchReady, GatewayMessage},
    payloads::{heartbeat_payload, resume_payload},
};

const URL: &str = "wss://gateway.discord.gg";
const PARAMS: &str = "?v=10&encoding=json";

type WSSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WSStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

#[derive(Debug, Clone)]
pub enum GatewayState {
    Connecting,
    Open,
    Resuming,
    Closed,
}

#[derive(Debug, Clone)]
pub enum GatewayEvent {
    ReconnectNeeded,
    Message(DispatchMessage),
}

#[derive(Debug, Clone)]
pub struct Gateway {
    inner: Arc<GatewayInner>,
}

#[derive(Debug)]
pub struct GatewayInner {
    state: RwLock<GatewayState>,
    connection_number: AtomicU32,
    sequence: AtomicU32,
    write: Mutex<WSSink>,
    token: String,
    resume_url: OnceCell<String>,
    session_id: OnceCell<String>,

    event_sender: mpsc::Sender<GatewayEvent>,
    event_receiver: Mutex<mpsc::Receiver<GatewayEvent>>,
    ready_sender: Mutex<Option<oneshot::Sender<Result<State>>>>,
}

impl Gateway {
    pub async fn new(token: String) -> Result<(Self, State)> {
        let (ws_stream, _) = tokio_tungstenite::connect_async(format!("{URL}/{PARAMS}")).await?;
        let (write, read) = ws_stream.split();

        info!("Connected to Gateway");

        let (event_sender, event_receiver) = mpsc::channel::<GatewayEvent>(10);
        let (ready_sender, ready_receiver) = oneshot::channel::<Result<State>>();

        let this = Self {
            inner: Arc::new(GatewayInner {
                state: RwLock::new(GatewayState::Connecting),
                connection_number: AtomicU32::new(0),
                sequence: AtomicU32::new(0),
                write: Mutex::new(write),
                token: token.clone(),
                resume_url: OnceCell::new(),
                session_id: OnceCell::new(),

                ready_sender: Mutex::new(Some(ready_sender)),
                event_sender,
                event_receiver: Mutex::new(event_receiver),
            }),
        };

        //Send Identify event
        this.send(identify_payload(&token)).await?;
        this.receive(read);

        let state = ready_receiver.await??;

        if let Some(user) = state.user_cache.get(&state.user_id) {
            info!("Gateway ready. Logged in as {}", user.username);
        } else {
            error!("Current user not in user cache");
        }

        Ok((this, state))
    }

    async fn resume(&self) -> Result<()> {
        if let GatewayState::Open = self.get_state().await {
            self.set_state(GatewayState::Resuming).await;

            let (ws_stream, _) = tokio_tungstenite::connect_async(format!(
                "{}/{PARAMS}",
                self.inner
                    .resume_url
                    .get()
                    .ok_or(anyhow!("Resume url was not set"))?
            ))
            .await?;
            let (write, read) = ws_stream.split();

            info!("Connected to Gateway");

            *(self.inner.write.lock().await) = write;
            self.inner.connection_number.fetch_add(1, Ordering::SeqCst);

            // Send resume event
            let payload = resume_payload(
                &self.inner.token,
                self.inner
                    .session_id
                    .get()
                    .ok_or(anyhow!("Session id was not set"))?,
                self.inner.sequence.load(Ordering::SeqCst),
            );
            self.send(payload).await?;

            self.receive(read);

            Ok(())
        } else {
            Err(anyhow!("Trying to resume gateway that is not open"))
        }
    }

    pub fn close(&self) {
        let this = self.clone();
        tokio::spawn(async move {
            this.set_state(GatewayState::Closed).await;

            // Close with code 1000
            let _ = this
                .inner
                .write
                .lock()
                .await
                .send(Message::Close(Some(CloseFrame {
                    code: CloseCode::Normal,
                    reason: Cow::Borrowed(""),
                })))
                .await;

            this.inner.event_receiver.lock().await.close();

            if let Some(sender) = this.inner.ready_sender.lock().await.take() {
                let _ = sender.send(Err(anyhow!("Gateway closed")));
            }
        });
    }

    pub fn subscribe(&self) -> Subscription<GatewayEvent> {
        subscription::unfold(self.inner.token.clone(), self.clone(), |this| async move {
            let event = this.inner.event_receiver.lock().await.recv().await;
            (event, this)
        })
    }

    fn receive(&self, read: WSStream) {
        let this = self.clone();

        tokio::spawn(async move {
            read.for_each(|message| async {
                match message {
                    Ok(message) => match message {
                        Message::Text(message) => {
                            if let Err(e) = this.process_message(&message).await {
                                error!("Failed to parse gateway message: {e}");
                            }
                        }
                        Message::Close(close) => {
                            info!("Gateway was closed");
                            if let GatewayState::Open = this.get_state().await {
                                let should_resume = if let Some(CloseFrame { code, .. }) = close {
                                    let code: u16 = code.into();
                                    if code == 4004 || (code >= 4010 && code <= 4014) {
                                        false
                                    } else {
                                        true
                                    }
                                } else {
                                    true
                                };

                                if should_resume {
                                    if let Err(e) = this.resume().await {
                                        error!("Failed to resume gateway: {e}");
                                    }
                                } else {
                                    this.close();
                                    if let Err(e) = this
                                        .inner
                                        .event_sender
                                        .send(GatewayEvent::ReconnectNeeded)
                                        .await
                                    {
                                        error!("Failed to send gateway event: {e}");
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    Err(e) => error!("Failed to receive gateway message: {e}"),
                }
            })
            .await;
        });
    }

    async fn process_message(&self, message: &str) -> Result<()> {
        let msg = serde_json::from_str::<GatewayMessage>(message)?;

        match msg.op {
            // Dispatch
            0 => {
                if let Some(s) = msg.sequence {
                    self.inner.sequence.store(s, Ordering::SeqCst);
                } else {
                    warn!("Gateway dispatch did not include sequence number");
                }

                let kind = msg
                    .kind
                    .as_ref()
                    .ok_or(anyhow!("Gateway dispatch did not include type"))?;

                self.process_dispatch(&kind, msg.data).await?;
            }
            // Heartbeat request
            1 => {
                let s = self.inner.sequence.load(Ordering::SeqCst);
                let payload = heartbeat_payload(if s > 0 { Some(s) } else { None });
                self.send(payload).await?
            }
            // Reconnect
            7 => self.inner.write.lock().await.close().await?,
            // Invalid Session
            9 => {
                self.close();
                self.inner
                    .event_sender
                    .send(GatewayEvent::ReconnectNeeded)
                    .await?;
            }
            // Hello
            10 => {
                // Heartbeat
                let ms = msg.data["heartbeat_interval"]
                    .as_u64()
                    .ok_or(anyhow!("Failed to parse heartbeat interval"))?;

                let this = self.clone();

                tokio::spawn(async move {
                    let connection_number = this.inner.connection_number.load(Ordering::SeqCst);

                    let mut interval = time::interval(Duration::from_millis(ms));
                    interval.tick().await;
                    interval.tick().await;

                    loop {
                        // Stop sending heartbeats when the gateway is closed or when there is a new connection
                        // (resuming a connection will send its own hello event)
                        if let GatewayState::Closed = this.get_state().await {
                            break;
                        } else if connection_number
                            != this.inner.connection_number.load(Ordering::SeqCst)
                        {
                            break;
                        }

                        let s = this.inner.sequence.load(Ordering::SeqCst);
                        let payload = heartbeat_payload(if s > 0 { Some(s) } else { None });
                        if let Err(e) = this.send(payload).await {
                            error!("Failed to send gateway heartbeat: {e}");
                            break;
                        }

                        interval.tick().await;
                    }
                });
            }
            // Heartbeat response
            11 => {}
            op => warn!("Unhandled gateway opcode {op}: {msg:?}"),
        }

        Ok(())
    }

    async fn process_dispatch(&self, kind: &str, data: Value) -> Result<()> {
        match kind {
            "READY" => {
                let data = serde_json::from_value::<DispatchReady>(data)?;

                self.inner
                    .resume_url
                    .set(data.resume_gateway_url.clone())
                    .map_err(|_| anyhow!("Could not set resume_gateway_url"))?;
                self.inner
                    .session_id
                    .set(data.session_id.clone())
                    .map_err(|_| anyhow!("Could not set session_id"))?;

                self.set_state(GatewayState::Open).await;

                if let Some(sender) = self.inner.ready_sender.lock().await.take() {
                    if let Err(_) = sender.send(Ok(data.into())) {
                        error!("Failed to send ready oneshot");
                    }
                }
            }
            "RESUMED" => {
                info!("Session successfully resumed");
                self.set_state(GatewayState::Open).await;
            }
            "MESSAGE_CREATE" | "MESSAGE_UPDATE" => {
                let data = serde_json::from_value::<DispatchMessage>(data)?;
                self.inner
                    .event_sender
                    .send(GatewayEvent::Message(data))
                    .await?;
            }
            msg_type => {
                warn!("Unhandled gateway dispatch type {msg_type}")
            }
        }

        Ok(())
    }

    async fn get_state(&self) -> GatewayState {
        self.inner.state.read().await.clone()
    }

    async fn set_state(&self, state: GatewayState) {
        *self.inner.state.write().await = state;
    }

    async fn send(&self, msg: String) -> Result<()> {
        self.inner
            .write
            .lock()
            .await
            .send(Message::Text(msg))
            .await?;

        Ok(())
    }
}
