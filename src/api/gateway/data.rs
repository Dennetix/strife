use serde::Deserialize;
use serde_json::Value;

use crate::data::{
    state::{Relationship, RelationshipKind, State},
    user::User,
};

#[derive(Debug, Clone, Deserialize)]
pub struct GatewayMessage {
    pub op: u32,
    #[serde(rename = "d")]
    pub data: Value,
    #[serde(rename = "t")]
    pub kind: Option<String>,
    #[serde(rename = "s")]
    pub sequence: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DispatchReady {
    pub user: User,
    pub relationships: Vec<RelationshipData>,
    pub resume_gateway_url: String,
    pub session_id: String,
}

impl Into<State> for DispatchReady {
    fn into(self) -> State {
        let relationships = self
            .relationships
            .iter()
            .map(|r| Relationship {
                id: r.user.id.clone(),
                kind: match r.kind {
                    1 => RelationshipKind::Friend,
                    2 => RelationshipKind::Blocked,
                    3 => RelationshipKind::PendingIncoming,
                    4 => RelationshipKind::PendingOutgoing,
                    5 => RelationshipKind::Implicit,
                    _ => RelationshipKind::None,
                },
            })
            .collect();

        let user_cache = self
            .relationships
            .into_iter()
            .map(|r| (r.user.id.clone(), r.user))
            .collect();

        State::new(self.user, relationships, user_cache)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RelationshipData {
    pub user: User,
    #[serde(rename = "type")]
    pub kind: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DispatchMessage {
    pub id: String,
    pub channel_id: String,
    pub author: User,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
}
