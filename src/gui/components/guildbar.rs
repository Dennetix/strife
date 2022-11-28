use iced::{
    alignment::Vertical,
    widget::{button, column, container, text},
    Element, Length,
};
use iced_lazy::Component;

use crate::gui::theme::Container;

#[derive(Debug, Clone)]
pub enum GuildbarEntry {
    SwitchTheme,
}

pub fn guildbar<Message>(
    on_select: impl Fn(GuildbarEntry) -> Message + 'static,
) -> Guildbar<Message> {
    Guildbar::new(on_select)
}

#[derive(Debug, Clone)]
pub enum GuildbarEvent {
    SwitchThemeClicked,
}

pub struct Guildbar<Message> {
    on_select: Box<dyn Fn(GuildbarEntry) -> Message>,
}

impl<Message> Guildbar<Message> {
    fn new(on_select: impl Fn(GuildbarEntry) -> Message + 'static) -> Self {
        Self {
            on_select: Box::new(on_select),
        }
    }
}

impl<Message, Renderer> Component<Message, Renderer> for Guildbar<Message>
where
    Renderer: iced_native::text::Renderer + 'static,
    Renderer::Theme: button::StyleSheet + text::StyleSheet + container::StyleSheet,
    <Renderer::Theme as container::StyleSheet>::Style: From<Container>,
{
    type State = ();
    type Event = GuildbarEvent;

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<Message> {
        Some((self.on_select)(GuildbarEntry::SwitchTheme))
    }

    fn view(&self, _state: &Self::State) -> iced_native::Element<'_, Self::Event, Renderer> {
        let settings = button(text("Switch")).on_press(GuildbarEvent::SwitchThemeClicked);

        container(column![settings])
            .height(Length::Fill)
            .align_y(Vertical::Bottom)
            .style(Container::BackgroundContrast2)
            .into()
    }
}

impl<'a, Message, Renderer> From<Guildbar<Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_native::text::Renderer + 'static,
    Renderer::Theme: text::StyleSheet + container::StyleSheet + button::StyleSheet,
    <Renderer::Theme as container::StyleSheet>::Style: From<Container>,
{
    fn from(guildbar: Guildbar<Message>) -> Self {
        iced_lazy::component(guildbar)
    }
}
