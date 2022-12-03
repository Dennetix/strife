use std::sync::Arc;

use crate::settings::Settings;

use super::{components::guildbar::View, views::settings::SettingsViewMessage};

#[derive(Debug, Clone)]
pub enum Message {
    SettingsLoaded(Settings),
    SettingsSaved(Result<(), Arc<anyhow::Error>>),
    ViewSelect(View),

    SettingsViewMessage(SettingsViewMessage),
}
