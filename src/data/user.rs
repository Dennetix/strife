use anyhow::Result;
use iced_native::image;
use serde::{Deserialize, Deserializer};

use crate::api::rest_client::REST_BASE_URL;

const DEFAULT_ACCENT_COLOR: u32 = 5793266;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    #[serde(deserialize_with = "parse_accent_color")]
    pub accent_color: u32,
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

fn parse_accent_color<'a, D: Deserializer<'a>>(d: D) -> Result<u32, D::Error> {
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or(DEFAULT_ACCENT_COLOR))
}
