use iced::Element;
use iced_graphics::Renderer;
use iced_lazy::{lazy, Component};
use iced_native::row;

use crate::{
    data::state::{RelationshipKind, State},
    gui::{
        components::sidebar::{sidebar, SidebarEntryType},
        theme::Theme,
    },
};

pub fn direct_messages_view<'a, Message>(
    state: &'a State,
    on_message: impl Fn(DirectMessagesViewMessage) -> Message + 'static,
) -> DirectMessagesView<'a, Message> {
    DirectMessagesView::new(state, on_message)
}

#[derive(Debug, Clone)]
pub enum DirectMessagesViewMessage {
    Default,
}

#[derive(Default)]
pub struct DirectMessagesState {
    active_channel: String,
}

#[derive(Debug, Clone)]
pub enum Event {
    ChannelSelected(SidebarEntryType<String>),
}

pub struct DirectMessagesView<'a, Message> {
    state: &'a State,
    on_message: Box<dyn Fn(DirectMessagesViewMessage) -> Message>,
}

impl<'a, Message> DirectMessagesView<'a, Message> {
    fn new(
        state: &'a State,
        on_message: impl Fn(DirectMessagesViewMessage) -> Message + 'static,
    ) -> Self {
        Self {
            state,
            on_message: Box::new(on_message),
        }
    }
}

impl<'a, Message, Backend> Component<Message, Renderer<Backend, Theme>>
    for DirectMessagesView<'a, Message>
where
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Text
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    type State = DirectMessagesState;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::ChannelSelected(SidebarEntryType::Button(channel, _)) => {
                state.active_channel = channel;
                None
            }
            _ => None,
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced_native::Element<'_, Self::Event, Renderer<Backend, Theme>> {
        let sidebar = lazy(self.state.relationships.len(), |_| {
            sidebar(
                &self
                    .state
                    .relationships
                    .iter()
                    .flat_map(|r| match r.kind {
                        RelationshipKind::Friend => {
                            if let Some(user) = self.state.user_cache.get(&r.id) {
                                Some(SidebarEntryType::User(user.clone()))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
                Event::ChannelSelected,
            )
        });

        row![sidebar].into()
    }
}

impl<'a, Message, Backend> From<DirectMessagesView<'a, Message>>
    for Element<'a, Message, Renderer<Backend, Theme>>
where
    Message: 'a,
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Text
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    fn from(direct_messages_view: DirectMessagesView<'a, Message>) -> Self {
        iced_lazy::component(direct_messages_view)
    }
}
