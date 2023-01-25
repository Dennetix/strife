use iced::{widget, Length};

pub mod guildbar;
pub mod images;
pub mod sidebar;

pub fn empty() -> widget::Space {
    widget::Space::new(Length::Shrink, Length::Shrink)
}
