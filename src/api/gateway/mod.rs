mod data;
mod payloads;

use std::{
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
use tokio::{
    net::TcpStream,
    sync::{
        oneshot::{self, Sender},
        Mutex,
    },
    time,
};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::{error, info, warn};
use url::Url;

use crate::{api::gateway::payloads::identify_payload, data::state::State};

use self::{
    data::{GatewayMessage, GatewayReadyData},
    payloads::heartbeat_payload,
};

const URL: &str = "wss://gateway.discord.gg/?v=9";

type WSSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WSStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

#[derive(Debug, Clone)]
pub struct Gateway {
    inner: Arc<GatewayInner>,
    closed: bool,
}

#[derive(Debug)]
pub struct GatewayInner {
    write: Mutex<WSSink>,
    sequence: AtomicU32,
    ready_sender: Mutex<Option<Sender<Result<State>>>>,
}

impl Gateway {
    pub async fn new(token: String) -> Result<(Self, State)> {
        let url = Url::parse(URL)?;

        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
        let (mut write, read) = ws_stream.split();

        info!("Connected to Gateway");

        // Identify
        let (ready_sender, ready_receiver) = oneshot::channel::<Result<State>>();
        write.send(Message::Text(identify_payload(&token))).await?;

        let this = Self {
            inner: Arc::new(GatewayInner {
                write: Mutex::new(write),
                sequence: AtomicU32::new(0),
                ready_sender: Mutex::new(Some(ready_sender)),
            }),
            closed: false,
        };

        this.receive(read);

        let state = ready_receiver.await??;

        info!("Gateway ready. Logged in as {}", state.user.username);

        Ok((this, state))
    }

    pub fn close(&mut self) {
        self.closed = true;

        let this = self.clone();
        tokio::spawn(async move {
            let _ = this.inner.write.lock().await.close().await;

            if let Some(sender) = this.inner.ready_sender.lock().await.take() {
                let _ = sender.send(Err(anyhow!("Gateway closed")));
            }

            info!("Gateway closed");
        });
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

                match kind.as_str() {
                    "READY" => {
                        let data = serde_json::from_value::<GatewayReadyData>(msg.data)?;

                        if let Some(sender) = self.inner.ready_sender.lock().await.take() {
                            if let Err(_) = sender.send(Ok(State { user: data.user })) {
                                error!("Failed to send ready oneshot");
                            }
                        }
                    }
                    msg_type => {
                        warn!("Unhandled gateway dispatch type {msg_type}")
                    }
                }
            }
            10 => {
                // Heartbeat
                let ms = msg.data["heartbeat_interval"]
                    .as_u64()
                    .ok_or(anyhow!("Failed to parse heartbeat interval"))?;

                let this = self.clone();

                tokio::spawn(async move {
                    let mut interval = time::interval(Duration::from_millis(ms));
                    interval.tick().await;

                    loop {
                        interval.tick().await;

                        let s = this.inner.sequence.load(Ordering::SeqCst);
                        let payload = heartbeat_payload(if s > 0 { Some(s) } else { None });
                        if let Err(_) = this.send(payload).await {
                            break;
                        }
                    }
                });
            }
            11 => {}
            op => warn!("Unhandled gateway opcode {op}: {msg:?}"),
        }

        Ok(())
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
