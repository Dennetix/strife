use std::sync::Arc;

use iced::widget::image;

use crate::{
    api::gateway::{Gateway, GatewayEvent},
    data::{settings::Settings, state::State, user::User},
};

use super::{
    components::guildbar::View,
    views::{direct_messages::DirectMessagesViewMessage, settings::SettingsViewMessage},
};

pub type Result<T> = core::result::Result<T, Arc<anyhow::Error>>;

pub fn map_result_message<T, Message>(
    f: impl FnOnce(Result<T>) -> Message + 'static,
) -> impl FnOnce(anyhow::Result<T>) -> Message + 'static {
    |r| match r {
        Ok(t) => f(Ok(t)),
        Err(e) => f(Err(Arc::new(e))),
    }
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    SettingsLoaded(Settings),
    SettingsSaved(Result<()>),
    AccountLoaded(Result<User>, Option<String>),
    AccountAvatarLoaded(String, Result<image::Handle>),
    GatewayConnected(Result<(Gateway, State)>),

    GatewayEvent(GatewayEvent),

    UserAvatarLoaded(String, Result<image::Handle>),

    ViewSelect(View),

    SettingsViewMessage(SettingsViewMessage),
    DirectMessagesViewMessage(DirectMessagesViewMessage),
}
