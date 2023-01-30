use iced::widget::scrollable::Properties;
use iced::{
    alignment::Vertical,
    widget::{button, container, horizontal_rule, scrollable, text, Column},
    Element, Length,
};
use iced_graphics::Renderer;
use iced_lazy::Component;
use iced_native::row;

use crate::data::state::{PrivateChannel, PrivateChannelKind};
use crate::data::user::User;
use crate::gui::theme::{Button, Container, Theme};

use super::images::{channel_icon, user_avatar};

#[derive(Debug, Clone, PartialEq)]
pub enum SidebarEntryType<T: Clone + PartialEq> {
    Button(T, String),
    PrivateChannel(PrivateChannel, Option<User>),
    Spacer,
}

fn sidebar_entry<'a, T, Backend>(
    selected: bool,
    entry_type: &SidebarEntryType<T>,
) -> Element<'a, SidebarEntryType<T>, Renderer<Backend, Theme>>
where
    T: Clone + PartialEq + 'a,
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Text
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    let content = match entry_type {
        SidebarEntryType::Button(_, label) => container(text(label))
            .height(Length::Units(20))
            .align_y(Vertical::Center),
        SidebarEntryType::PrivateChannel(channel, user) => container(
            match channel.kind {
                PrivateChannelKind::DirectMessage => {
                    if let Some(user) = user {
                        row![
                            user_avatar(user, 25),
                            text(format!("{:?} {}", user.presence, user.username))
                        ]
                    } else {
                        row![text("Error finding user")]
                    }
                }
                PrivateChannelKind::Group => row![
                    channel_icon(channel.icon_handle.clone(), 25),
                    text(if let Some(name) = &channel.name {
                        name.clone()
                    } else {
                        format!("{} Members", channel.recipients.len() + 1)
                    })
                ],
            }
            .spacing(10),
        )
        .height(Length::Units(25))
        .align_y(Vertical::Center),
        SidebarEntryType::Spacer => return horizontal_rule(15).into(),
    };

    button(content)
        .style(Button::TransparentHover(selected, Some(5.0)))
        .width(Length::Fill)
        .padding(10)
        .on_press(entry_type.clone())
        .into()
}

pub fn sidebar<T: Clone + PartialEq, Message>(
    entries: &[SidebarEntryType<T>],
    on_select: impl Fn(SidebarEntryType<T>) -> Message + 'static,
) -> Sidebar<T, Message> {
    Sidebar::new(entries, on_select)
}

pub struct State<T: Clone + PartialEq> {
    active_entry: Option<SidebarEntryType<T>>,
}

impl<T: Clone + PartialEq> Default for State<T> {
    fn default() -> Self {
        Self { active_entry: None }
    }
}

pub struct Sidebar<T: Clone + PartialEq, Message> {
    entries: Vec<SidebarEntryType<T>>,
    on_select: Box<dyn Fn(SidebarEntryType<T>) -> Message>,
}

impl<T: Clone + PartialEq, Message> Sidebar<T, Message> {
    fn new(
        entries: &[SidebarEntryType<T>],
        on_select: impl Fn(SidebarEntryType<T>) -> Message + 'static,
    ) -> Self {
        Self {
            entries: entries.to_vec(),
            on_select: Box::new(on_select),
        }
    }
}

impl<T, Message, Backend> Component<Message, Renderer<Backend, Theme>> for Sidebar<T, Message>
where
    T: Clone + PartialEq,
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Text
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    type State = State<T>;
    type Event = SidebarEntryType<T>;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        state.active_entry = Some(event.clone());
        Some((self.on_select)(event))
    }

    fn view(&self, state: &Self::State) -> Element<'_, Self::Event, Renderer<Backend, Theme>> {
        let entries = self
            .entries
            .iter()
            .map(|entry| {
                let selected = if let Some(active_entry) = &state.active_entry {
                    *active_entry == *entry
                } else {
                    if let Some(first) = self.entries.first() {
                        *first == *entry
                    } else {
                        false
                    }
                };
                sidebar_entry(selected, entry).into()
            })
            .collect();

        let scrollable = scrollable(
            Column::with_children(entries)
                .width(Length::Fill)
                .spacing(5)
                .padding([15, 13, 15, 10]),
        )
        .vertical_scroll(Properties::new().width(5).scroller_width(5).margin(5));

        container(scrollable)
            .style(Container::BackgroundStrong1(0.0))
            .width(Length::Units(230))
            .height(Length::Fill)
            .into()
    }
}

impl<'a, T, Message, Backend> From<Sidebar<T, Message>>
    for Element<'a, Message, Renderer<Backend, Theme>>
where
    T: Clone + PartialEq + 'static,
    Message: 'a,
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Text
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    fn from(sidebar: Sidebar<T, Message>) -> Self {
        iced_lazy::component(sidebar)
    }
}
