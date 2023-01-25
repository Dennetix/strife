use std::collections::HashMap;

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
    pub current_user: User,
    pub relationships: Vec<Relationship>,
    pub user_cache: HashMap<String, User>,
    pub message_cache: HashMap<String, Vec<Message>>,
}

impl State {
    pub fn new(
        current_user: User,
        relationships: Vec<Relationship>,
        user_cache: HashMap<String, User>,
    ) -> Self {
        State {
            current_user,
            relationships,
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
