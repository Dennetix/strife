use iced::Element;
use iced_graphics::Renderer;
use iced_lazy::Component;
use iced_native::row;

use crate::{
    data::state::{PrivateChannelKind, State},
    gui::{
        components::sidebar::{sidebar, SidebarEntryType},
        theme::Theme,
    },
};

pub fn private_channels_view<'a, Message>(
    state: &'a State,
    on_message: impl Fn(PrivateChannelsViewMessage) -> Message + 'static,
) -> PrivateChannelsView<'a, Message> {
    PrivateChannelsView::new(state, on_message)
}

#[derive(Debug, Clone)]
pub enum PrivateChannelsViewMessage {
    Default,
}

#[derive(Default)]
pub struct PrivateChannelsState {
    active_channel: String,
}

#[derive(Debug, Clone)]
pub enum Event {
    ChannelSelected(SidebarEntryType<String>),
}

pub struct PrivateChannelsView<'a, Message> {
    state: &'a State,
    on_message: Box<dyn Fn(PrivateChannelsViewMessage) -> Message>,
}

impl<'a, Message> PrivateChannelsView<'a, Message> {
    fn new(
        state: &'a State,
        on_message: impl Fn(PrivateChannelsViewMessage) -> Message + 'static,
    ) -> Self {
        Self {
            state,
            on_message: Box::new(on_message),
        }
    }
}

impl<'a, Message, Backend> Component<Message, Renderer<Backend, Theme>>
    for PrivateChannelsView<'a, Message>
where
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Text
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    type State = PrivateChannelsState;
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
        let sidebar = sidebar(
            &self
                .state
                .private_channels
                .iter()
                .flat_map(|c| match c.kind {
                    PrivateChannelKind::DirectMessage => {
                        if let Some(user) = self
                            .state
                            .user_cache
                            .get(c.recipients.first().unwrap_or(&String::from("")))
                        {
                            Some(SidebarEntryType::User(user.clone()))
                        } else {
                            None
                        }
                    }
                    PrivateChannelKind::Group => Some(SidebarEntryType::Group(c.clone())),
                })
                .collect::<Vec<_>>(),
            Event::ChannelSelected,
        );

        row![sidebar].into()
    }
}

impl<'a, Message, Backend> From<PrivateChannelsView<'a, Message>>
    for Element<'a, Message, Renderer<Backend, Theme>>
where
    Message: 'a,
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Text
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    fn from(direct_messages_view: PrivateChannelsView<'a, Message>) -> Self {
        iced_lazy::component(direct_messages_view)
    }
}
