use iced::{
    widget::{column, scrollable, text},
    Element,
};
use iced_graphics::Renderer;
use iced_lazy::Component;

use crate::{data::state::State, gui::theme::Theme};

use self::message::message;

mod message;

pub fn text_chat<'a, Message>(
    channel_id: String,
    state: &'a State,
    on_message: impl Fn(TextChatMessage) -> Message + 'static,
) -> TextChat<'a, Message> {
    TextChat::new(channel_id, state, on_message)
}

#[derive(Debug, Clone)]
pub enum TextChatMessage {
    Default,
}

#[derive(Default)]
pub struct TextChatState;

#[derive(Debug, Clone)]
pub enum TextChatEvent {
    Default,
}

pub struct TextChat<'a, Message> {
    channel_id: String,
    state: &'a State,
    on_message: Box<dyn Fn(TextChatMessage) -> Message>,
}

impl<'a, Message> TextChat<'a, Message> {
    fn new(
        channel_id: String,
        state: &'a State,
        on_message: impl Fn(TextChatMessage) -> Message + 'static,
    ) -> Self {
        Self {
            channel_id,
            state,
            on_message: Box::new(on_message),
        }
    }
}

impl<'a, Message, Backend> Component<Message, Renderer<Backend, Theme>> for TextChat<'a, Message>
where
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Text
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    type State = TextChatState;
    type Event = TextChatEvent;

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<Message> {
        None
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced_native::Element<'_, Self::Event, Renderer<Backend, Theme>> {
        let messages = self.state.message_cache.get(&self.channel_id);
        if let Some(messages) = messages {
            scrollable(column(
                messages
                    .into_iter()
                    .map(|m| message(m, &self.state.user_cache))
                    .collect(),
            ))
            .into()
        } else {
            text("No messages").into()
        }
    }
}

impl<'a, Message, Backend> From<TextChat<'a, Message>>
    for Element<'a, Message, Renderer<Backend, Theme>>
where
    Message: 'a,
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Text
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    fn from(text_chat: TextChat<'a, Message>) -> Self {
        iced_lazy::component(text_chat)
    }
}
