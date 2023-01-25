use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use crate::data::{
    state::{PrivateChannel, PrivateChannelKind, Relationship, RelationshipKind, State},
    user::{Presence, User},
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
    pub private_channels: Vec<PrivateChannelData>,
    pub presences: Vec<PresenceData>,
    pub resume_gateway_url: String,
    pub session_id: String,
}

impl Into<State> for DispatchReady {
    fn into(self) -> State {
        // Create vector of relationships
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

        // Create vector of private channels
        let private_channels = self
            .private_channels
            .into_iter()
            .map(|c| PrivateChannel {
                id: c.id,
                kind: match c.kind {
                    3 => PrivateChannelKind::Group,
                    _ => PrivateChannelKind::DirectMessage,
                },
                recipients: c.recipients.into_iter().map(|r| r.id).collect(),
                owner_id: c.owner_id,
                name: c.name,
                icon: c.icon,
                icon_handle: None,
            })
            .collect();

        // Put all known users (from relationships) into a HashMap
        let mut user_cache = self
            .relationships
            .into_iter()
            .map(|r| (r.user.id.clone(), r.user))
            .collect::<HashMap<String, User>>();

        // Update users presences
        self.presences.into_iter().for_each(|p| {
            if let Some(user) = user_cache.get_mut(&p.user.id) {
                user.presence = match p.status.as_str() {
                    "online" => Presence::Online,
                    "idle" => Presence::Idle,
                    "dnd" => Presence::DoNotDisturb,
                    _ => Presence::Offline,
                }
            }
        });

        State::new(self.user, relationships, private_channels, user_cache)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RelationshipData {
    pub user: User,
    #[serde(rename = "type")]
    pub kind: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PrivateChannelData {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: u16,
    pub recipients: Vec<User>,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PresenceData {
    pub user: User,
    pub status: String,
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
