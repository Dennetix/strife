use iced::{
    color,
    widget::{container, image, svg},
    Color, Element, Length,
};
use iced_graphics::Renderer;

use crate::{
    data::user::{User, DEFAULT_ACCENT_COLOR},
    gui::{
        icons,
        theme::{Container, Theme},
    },
};

pub fn user_avatar<'a, Message, Backend>(
    user: &User,
    size: u16,
) -> Element<'a, Message, Renderer<Backend, Theme>>
where
    Message: 'a,
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    let image: Element<'a, Message, Renderer<Backend, Theme>> =
        if let Some(handle) = &user.avatar_handle {
            image(handle.clone()).into()
        } else {
            container(
                svg(icons::USER.clone())
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .style(Container::Color(
                color!(user.accent_color.unwrap_or(DEFAULT_ACCENT_COLOR)),
                size as f32 / 2.0,
            ))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding((size as f32 * 0.15) as u16)
            .into()
        };

    container(image)
        .style(Container::Color(Color::TRANSPARENT, size as f32 / 2.0))
        .width(Length::Units(size))
        .height(Length::Units(size))
        .into()
}
