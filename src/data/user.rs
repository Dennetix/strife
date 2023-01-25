use anyhow::Result;
use iced_native::image;
use serde::Deserialize;

use crate::api::rest_client::REST_BASE_URL;

pub const DEFAULT_ACCENT_COLOR: u32 = 5793266;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub accent_color: Option<u32>,
    pub avatar: Option<String>,
    #[serde(skip)]
    pub avatar_handle: Option<image::Handle>,
}

impl User {
    pub async fn from_token(token: String) -> Result<Self> {
        let data = reqwest::Client::new()
            .get(format!("{REST_BASE_URL}/users/@me"))
            .header("Authorization", token)
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str::<Self>(&data)?)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
