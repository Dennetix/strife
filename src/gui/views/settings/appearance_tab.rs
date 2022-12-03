use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, container, text, vertical_space},
    Color, Element, Length,
};
use iced_graphics::Renderer;
use iced_lazy::Component;
use iced_native::{column, row};

use crate::gui::theme::{
    data::{DefaultThemes, ThemeData},
    Button, Container, Text, Theme,
};

use super::Event;

struct ThemeButton<Message> {
    selected: bool,
    theme: ThemeData,
    on_press: Box<dyn Fn(String) -> Message>,
}

impl<Message> ThemeButton<Message> {
    fn new(
        selected: bool,
        theme: ThemeData,
        on_press: impl Fn(String) -> Message + 'static,
    ) -> Self {
        Self {
            selected,
            theme,
            on_press: Box::new(on_press),
        }
    }
}

impl<Message, Backend> Component<Message, Renderer<Backend, Theme>> for ThemeButton<Message>
where
    Backend: iced_graphics::Backend + iced_graphics::backend::Text + 'static,
{
    type State = ();
    type Event = ();

    fn update(&mut self, _state: &mut Self::State, _event: Self::Event) -> Option<Message> {
        Some((self.on_press)(self.theme.id.clone()))
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer<Backend, Theme>> {
        button(
            container(row![
                container(vertical_space(Length::Shrink))
                    .style(Container::Color(
                        Color::from(self.theme.theme.background_strong2),
                        10.0,
                    ))
                    .width(Length::Units(40))
                    .height(Length::Fill),
                container(
                    text(self.theme.name.clone())
                        .style(Text::Color(Color::from(self.theme.theme.text)))
                        .size(18)
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .padding([5, 10, 10, 10])
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
            ])
            .style(Container::Color(
                Color::from(self.theme.theme.background),
                11.0,
            ))
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .style(Button::TransparentBorder(self.selected, Some(15.0), 5.0))
        .width(Length::Units(180))
        .height(Length::Units(110))
        .padding(5)
        .on_press(())
        .into()
    }
}

impl<'a, Message, Backend> From<ThemeButton<Message>>
    for Element<'a, Message, Renderer<Backend, Theme>>
where
    Message: 'a,
    Backend: iced_graphics::Backend + iced_graphics::backend::Text + 'static,
{
    fn from(button: ThemeButton<Message>) -> Self {
        iced_lazy::component(button)
    }
}

pub fn appearance_tab<'a, Backend>(
    active_theme: &str,
) -> Element<'a, Event, Renderer<Backend, Theme>>
where
    Backend: iced_graphics::Backend + iced_graphics::backend::Text + 'static,
{
    let dark = ThemeData::dark();
    let light = ThemeData::light();

    let default_themes = container(
        row![
            ThemeButton::new(dark.id == active_theme, dark, Event::ThemeSelected),
            ThemeButton::new(light.id == active_theme, light, Event::ThemeSelected)
        ]
        .spacing(5),
    )
    .style(Container::BackgroundWeak(20.0))
    .padding(12);

    column![text("Theme"), default_themes].spacing(10).into()
}
