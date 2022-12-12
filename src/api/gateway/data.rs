use serde::Deserialize;
use serde_json::Value;

use crate::data::user::User;

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
    pub resume_gateway_url: String,
    pub session_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DispatchMessage {
    pub id: String,
    pub channel_id: String,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
}
