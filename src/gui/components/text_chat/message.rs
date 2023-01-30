use std::collections::HashMap;

use iced::{widget::text, Element};
use iced_graphics::Renderer;

use crate::{
    data::{state::Message as ChatMessage, user::User},
    gui::theme::Theme,
};

pub fn message<'a, Message, Backend>(
    message: &'a ChatMessage,
    user_cache: &'a HashMap<String, User>,
) -> Element<'a, Message, Renderer<Backend, Theme>>
where
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Text
        + iced_graphics::backend::Image
        + 'static,
{
    match message {
        ChatMessage::Default { user_id, content } => {
            let user = user_cache.get(user_id);
            text(format!(
                "{}: {}",
                if let Some(user) = user {
                    &user.username
                } else {
                    "User not found"
                },
                content
            ))
            .into()
        }
    }
}
