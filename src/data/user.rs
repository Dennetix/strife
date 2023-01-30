use anyhow::Result;
use iced_native::image;
use serde::{
    de::{self},
    Deserialize, Deserializer,
};

use crate::api::rest_client::REST_BASE_URL;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(deserialize_with = "str_to_u16")]
    pub discriminator: u16,
    pub accent_color: Option<u32>,
    pub avatar: Option<String>,
    #[serde(skip)]
    pub avatar_handle: Option<image::Handle>,
    #[serde(skip)]
    pub presence: Presence,
}

fn str_to_u16<'a, D: Deserializer<'a>>(deserializer: D) -> Result<u16, D::Error> {
    String::deserialize(deserializer)?
        .parse()
        .map_err(de::Error::custom)
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

#[derive(Default, Debug, Clone)]
pub enum Presence {
    #[default]
    Offline,
    Online,
    Idle,
    DoNotDisturb,
}
