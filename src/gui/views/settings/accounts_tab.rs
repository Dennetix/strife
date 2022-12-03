use iced::{widget::text, Element};
use iced_graphics::Renderer;

use crate::gui::theme::Theme;

use super::Event;

pub fn accounts_tab<'a, Backend>() -> Element<'a, Event, Renderer<Backend, Theme>>
where
    Backend: iced_graphics::Backend + iced_graphics::backend::Text + 'static,
{
    text("Accounts").into()
}
