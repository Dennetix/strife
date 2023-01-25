use anyhow::Result;
use iced_native::image;

const CDN_BASE_URL: &str = "https://cdn.discordapp.com";

#[derive(Default, Clone)]
pub struct CdnClient {
    client: reqwest::Client,
}

impl CdnClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn avatar(self, user_id: String, avatar: String, size: u16) -> Result<image::Handle> {
        let data = self
            .client
            .get(format!(
                "{CDN_BASE_URL}/avatars/{user_id}/{avatar}.png?size={size}"
            ))
            .send()
            .await?
            .bytes()
            .await?;

        Ok(image::Handle::from_memory(data.to_vec()))
    }

    pub async fn default_avatar(self, discriminator: u16) -> Result<image::Handle> {
        let data = self
            .client
            .get(format!(
                "{CDN_BASE_URL}/embed/avatars/{}.png?size=16",
                discriminator % 5
            ))
            .send()
            .await?
            .bytes()
            .await?;

        Ok(image::Handle::from_memory(data.to_vec()))
    }

    pub async fn channel_icon(
        self,
        channel_id: String,
        icon: String,
        size: u16,
    ) -> Result<image::Handle> {
        let data = self
            .client
            .get(format!(
                "{CDN_BASE_URL}/channel-icons/{channel_id}/{icon}.png?size={size}"
            ))
            .send()
            .await?
            .bytes()
            .await?;

        Ok(image::Handle::from_memory(data.to_vec()))
    }
}
