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
pub struct GatewayReadyData {
    pub user: User,
}
