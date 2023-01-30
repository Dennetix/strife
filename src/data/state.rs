use std::collections::HashMap;

use iced::widget::image;

use crate::api::gateway::Gateway;

use super::user::User;

#[derive(Debug, Clone)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connecetd(State, Gateway),
}

#[derive(Debug, Clone)]
pub struct State {
    pub user_id: String,
    pub relationships: Vec<Relationship>,
    pub private_channels: Vec<PrivateChannel>,
    pub user_cache: HashMap<String, User>,
    pub message_cache: HashMap<String, Vec<Message>>,
}

impl State {
    pub fn new(
        user_id: String,
        relationships: Vec<Relationship>,
        private_channels: Vec<PrivateChannel>,
        user_cache: HashMap<String, User>,
    ) -> Self {
        State {
            user_id,
            relationships,
            private_channels,
            user_cache,
            message_cache: HashMap::with_capacity(50),
        }
    }

    pub fn insert_message(&mut self, channel_id: String, msg: Message) {
        self.message_cache
            .entry(channel_id)
            .and_modify(|msgs| msgs.push(msg.clone()))
            .or_insert_with(|| {
                let mut v = Vec::with_capacity(50);
                v.push(msg);
                v
            });
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Default { user_id: String, content: String },
}

#[derive(Debug, Clone)]
pub enum RelationshipKind {
    None,
    Friend,
    Blocked,
    PendingIncoming,
    PendingOutgoing,
    Implicit,
}

#[derive(Debug, Clone)]
pub struct Relationship {
    pub id: String,
    pub kind: RelationshipKind,
}

#[derive(Debug, Clone)]
pub enum PrivateChannelKind {
    DirectMessage,
    Group,
}

#[derive(Debug, Clone)]
pub struct PrivateChannel {
    pub id: String,
    pub kind: PrivateChannelKind,
    pub recipients: Vec<String>,
    pub owner_id: Option<String>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub icon_handle: Option<image::Handle>,
    pub last_message_timestamp: u64,
}

impl PartialEq for PrivateChannel {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
