use std::path::PathBuf;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::{error, warn};

use crate::gui::theme::data::{DefaultThemes, ThemeData};

pub fn config_path() -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        Some(config_dir.join("strife"))
    } else {
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "camelCase")]
pub struct Settings {
    pub theme: String,
    pub active_account: String,
    pub accounts: Vec<String>,
}

impl Settings {
    pub async fn load() -> Self {
        if let Some(path) = config_path() {
            if let Ok(data) = fs::read(path.join("settings.json")).await {
                if let Ok(settings) = serde_json::from_slice::<Self>(&data) {
                    return settings;
                } else {
                    error!("Failed to parse settings. Using default settings");
                }
            } else {
                warn!("Failed to load settings file. Using default settings. (This is normal if settings have not been changed yet)");
            }
        } else {
            error!("Failed to get the platforms config path. Using default settings");
        }

        Default::default()
    }

    pub async fn save(self) -> Result<()> {
        let path = config_path().ok_or(anyhow!("Failed to get config path"))?;

        fs::create_dir_all(&path).await?;

        fs::write(
            path.join("settings.json"),
            serde_json::to_string(&self)?.as_bytes(),
        )
        .await?;

        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: ThemeData::dark().id,
            active_account: String::from(""),
            accounts: vec![],
        }
    }
}
