use iced::widget::scrollable::Properties;
use iced::{
    alignment::Horizontal,
    widget::{button, column, container, horizontal_rule, scrollable, svg, vertical_space},
    Element, Length,
};
use iced_graphics::Renderer;
use iced_lazy::Component;

use crate::gui::{
    icons,
    theme::{Button, Container, Rule, Scrollable, Theme},
};

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    DirectMessages,
    Settings,
}

pub fn guildbar<Message>(
    active_view: View,
    on_select: impl Fn(View) -> Message + 'static,
) -> Guildbar<Message> {
    Guildbar::new(active_view, on_select)
}

#[derive(Debug, Clone)]
pub enum GuildbarEvent {
    DirectMessagesPressed,
    SettingsPressed,
}

pub struct Guildbar<Message> {
    active_view: View,
    on_select: Box<dyn Fn(View) -> Message>,
}

impl<Message> Guildbar<Message> {
    fn new(active_view: View, on_select: impl Fn(View) -> Message + 'static) -> Self {
        Self {
            active_view,
            on_select: Box::new(on_select),
        }
    }
}

impl<Message, Backend> Component<Message, Renderer<Backend, Theme>> for Guildbar<Message>
where
    Backend: iced_graphics::Backend + iced_graphics::backend::Svg + 'static,
{
    type State = ();
    type Event = GuildbarEvent;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            GuildbarEvent::DirectMessagesPressed => Some((self.on_select)(View::DirectMessages)),
            GuildbarEvent::SettingsPressed => Some((self.on_select)(View::Settings)),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer<Backend, Theme>> {
        let private_channels_button = button(svg(icons::PRIVATE_CHANNELS.clone()))
            .style(Button::Primary(Some(15.0)))
            .width(Length::Units(51))
            .height(Length::Units(51))
            .padding(15)
            .on_press(GuildbarEvent::DirectMessagesPressed);

        let guilds = scrollable(
            column![
                private_channels_button,
                horizontal_rule(2).style(Rule::Width(2, 60.0)),
                vertical_space(Length::Units(750))
            ]
            .spacing(10),
        )
        .style(Scrollable::Weak)
        .height(Length::Fill)
        .vertical_scroll(Properties::new().width(5).scroller_width(5));

        let settings_button = button(svg(icons::SETTINGS.clone()))
            .style(Button::TransparentHover(
                self.active_view == View::Settings,
                Some(17.5),
            ))
            .width(Length::Units(35))
            .height(Length::Units(35))
            .padding(6)
            .on_press(GuildbarEvent::SettingsPressed);

        container(
            column![
                guilds,
                horizontal_rule(2).style(Rule::Width(2, 60.0)),
                container(settings_button)
                    .width(Length::Fill)
                    .align_x(Horizontal::Center)
            ]
            .spacing(10),
        )
        .style(Container::BackgroundStrong2(0.0))
        .width(Length::Units(75))
        .height(Length::Fill)
        .padding(12)
        .into()
    }
}

impl<'a, Message, Backend> From<Guildbar<Message>>
    for Element<'a, Message, Renderer<Backend, Theme>>
where
    Message: 'a,
    Backend: iced_graphics::Backend + iced_graphics::backend::Svg + 'static,
{
    fn from(guildbar: Guildbar<Message>) -> Self {
        iced_lazy::component(guildbar)
    }
}
