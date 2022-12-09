use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, container, text, vertical_space},
    Color, Element, Length,
};
use iced_graphics::Renderer;
use iced_native::{column, row};

use crate::gui::theme::{
    data::{DefaultThemes, ThemeData},
    Button, Container, Text, Theme,
};

use super::Event;

fn theme_button<'a, Backend>(
    selected: bool,
    theme: ThemeData,
) -> Element<'a, Event, Renderer<Backend, Theme>>
where
    Backend: iced_graphics::Backend + iced_graphics::backend::Text + 'static,
{
    let content = container(row![
        container(vertical_space(Length::Shrink))
            .style(Container::Color(
                Color::from(theme.theme.background_strong2),
                9.0,
            ))
            .width(Length::Units(40))
            .height(Length::Fill),
        container(
            text(theme.name.clone())
                .style(Text::Color(Color::from(theme.theme.text)))
                .size(18)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([5, 10, 10, 10])
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
    ])
    .style(Container::Color(Color::from(theme.theme.background), 9.0))
    .width(Length::Fill)
    .height(Length::Fill);

    button(content)
        .style(Button::Border(selected, Some(15.0), 5.0))
        .width(Length::Units(160))
        .height(Length::Units(95))
        .padding(5)
        .on_press(Event::ThemeSelected(theme.id))
        .into()
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
            theme_button(dark.id == active_theme, dark),
            theme_button(light.id == active_theme, light)
        ]
        .spacing(5),
    )
    .style(Container::BackgroundWeak(20.0))
    .padding(12);

    column![text("Theme"), default_themes].spacing(15).into()
}
