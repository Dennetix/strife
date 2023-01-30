use iced::{widget::text, Element};
use iced_graphics::Renderer;
use iced_lazy::Component;
use iced_native::row;

use crate::{
    data::state::{PrivateChannel, PrivateChannelKind, State},
    gui::{
        components::{
            sidebar::{sidebar, SidebarEntryType},
            text_chat::{text_chat, TextChatMessage},
        },
        theme::Theme,
    },
};

pub fn private_channels_view<'a, Message>(
    state: &'a State,
    on_message: impl Fn(PrivateChannelsViewMessage) -> Message + 'static,
) -> PrivateChannelsView<'a, Message> {
    PrivateChannelsView::new(state, on_message)
}

#[derive(Default, Debug, Clone)]
pub enum Tab {
    #[default]
    Friends,
    Channel(String),
}

#[derive(Debug, Clone)]
pub enum PrivateChannelsViewMessage {
    Default,
}

#[derive(Default)]
pub struct PrivateChannelsState {
    active_tab: Tab,
}

#[derive(Debug, Clone)]
pub enum Event {
    TabSelected(SidebarEntryType<()>),
    TextChatMessage(TextChatMessage),
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
            Event::TabSelected(entry_type) => match entry_type {
                SidebarEntryType::PrivateChannel(PrivateChannel { id, .. }, _) => {
                    state.active_tab = Tab::Channel(id)
                }
                _ => state.active_tab = Tab::Friends,
            },
            Event::TextChatMessage(_) => {}
        }
        None
    }

    fn view(
        &self,
        state: &Self::State,
    ) -> iced_native::Element<'_, Self::Event, Renderer<Backend, Theme>> {
        let sidebar = sidebar(
            &[
                vec![
                    SidebarEntryType::Button((), String::from("Friends")),
                    SidebarEntryType::Spacer,
                ],
                self.state
                    .private_channels
                    .iter()
                    .flat_map(|c| match c.kind {
                        PrivateChannelKind::DirectMessage => {
                            Some(SidebarEntryType::PrivateChannel(
                                c.clone(),
                                self.state
                                    .user_cache
                                    .get(c.recipients.first().unwrap_or(&String::from("")))
                                    .cloned(),
                            ))
                        }
                        PrivateChannelKind::Group => {
                            Some(SidebarEntryType::PrivateChannel(c.clone(), None))
                        }
                    })
                    .collect::<Vec<_>>(),
            ]
            .concat(),
            Event::TabSelected,
        );

        let content: Element<_, _> = match &state.active_tab {
            Tab::Friends => text("Friends...").into(),
            Tab::Channel(id) => text_chat(id.clone(), &self.state, Event::TextChatMessage).into(),
        };

        row![sidebar, content].into()
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
