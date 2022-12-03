use iced::{
    alignment::Vertical,
    widget::{button, container, horizontal_rule, scrollable, text, Column},
    Element, Length,
};
use iced_graphics::Renderer;
use iced_lazy::Component;

use crate::gui::theme::{Button, Container, Theme};

#[derive(Debug, Clone, PartialEq)]
pub enum SidebarEntryType<T: Clone + PartialEq> {
    Button(T, &'static str),
    Spacer,
}

struct SidebarEntry<'a, T: Clone + PartialEq, Message> {
    selected: bool,
    entry_type: &'a SidebarEntryType<T>,
    on_press: Box<dyn Fn(SidebarEntryType<T>) -> Message>,
}

impl<'a, T: Clone + PartialEq, Message> SidebarEntry<'a, T, Message> {
    fn new(
        selected: bool,
        entry_type: &'a SidebarEntryType<T>,
        on_press: impl Fn(SidebarEntryType<T>) -> Message + 'static,
    ) -> Self {
        Self {
            selected,
            entry_type,
            on_press: Box::new(on_press),
        }
    }
}

impl<'a, T, Message, Backend> Component<Message, Renderer<Backend, Theme>>
    for SidebarEntry<'a, T, Message>
where
    T: Clone + PartialEq,
    Backend: iced_graphics::Backend + iced_graphics::backend::Text + 'static,
{
    type State = ();
    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<Message> {
        Some((self.on_press)(self.entry_type.clone()))
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer<Backend, Theme>> {
        let content = match &self.entry_type {
            SidebarEntryType::Button(_, label) => text(label),
            SidebarEntryType::Spacer => return horizontal_rule(15).into(),
        };

        button(
            container(content)
                .height(Length::Units(20))
                .align_y(Vertical::Center),
        )
        .style(Button::TransparentHover(self.selected, Some(5.0)))
        .width(Length::Fill)
        .padding(10)
        .on_press(())
        .into()
    }
}

impl<'a, T, Message, Backend> From<SidebarEntry<'a, T, Message>>
    for Element<'a, Message, Renderer<Backend, Theme>>
where
    T: Clone + PartialEq + 'a,
    Message: 'a,
    Backend: iced_graphics::Backend + iced_graphics::backend::Text + 'static,
{
    fn from(sidebar_entry: SidebarEntry<'a, T, Message>) -> Self {
        iced_lazy::component(sidebar_entry)
    }
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
    Backend: iced_graphics::Backend + iced_graphics::backend::Text + 'static,
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
                SidebarEntry::new(selected, entry, |e| e).into()
            })
            .collect();

        let scrollable = scrollable(
            Column::with_children(entries)
                .width(Length::Fill)
                .spacing(5)
                .padding([15, 13, 15, 10]),
        )
        .style((2.5, false))
        .scrollbar_margin(5)
        .scrollbar_width(5)
        .scroller_width(5);

        container(scrollable)
            .style(Container::BackgroundStrong1)
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
    Backend: iced_graphics::Backend + iced_graphics::backend::Text + 'static,
{
    fn from(sidebar: Sidebar<T, Message>) -> Self {
        iced_lazy::component(sidebar)
    }
}
