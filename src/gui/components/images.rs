use iced::{
    color,
    widget::{container, image, svg},
    Color, Element, Length,
};
use iced_graphics::Renderer;

use crate::{
    data::user::User,
    gui::{
        icons,
        theme::{Container, Theme},
    },
};

use super::empty;

pub const DEFAULT_ACCENT_COLOR: u32 = 5793266;

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
    let image: Element<_, _> = if let Some(handle) = &user.avatar_handle {
        image(handle.clone()).into()
    } else {
        empty().into()
    };

    container(image)
        .style(Container::Color(Color::TRANSPARENT, size as f32 / 2.0))
        .width(Length::Units(size))
        .height(Length::Units(size))
        .into()
}

pub fn channel_icon<'a, Message, Backend>(
    icon: Option<image::Handle>,
    size: u16,
) -> Element<'a, Message, Renderer<Backend, Theme>>
where
    Message: 'a,
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    let image: Element<'a, Message, Renderer<Backend, Theme>> = if let Some(handle) = icon {
        image(handle).into()
    } else {
        container(
            svg(icons::USERS.clone())
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .style(Container::Color(
            color!(DEFAULT_ACCENT_COLOR),
            size as f32 / 2.0,
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .padding((size as f32 * 0.2).round() as u16)
        .into()
    };

    container(image)
        .style(Container::Color(Color::TRANSPARENT, size as f32 / 2.0))
        .width(Length::Units(size))
        .height(Length::Units(size))
        .into()
}
