use iced::{widget, Length};

pub mod guildbar;
pub mod sidebar;

mod user_avatar;
pub use user_avatar::user_avatar;

pub fn empty() -> widget::Space {
    widget::Space::new(Length::Shrink, Length::Shrink)
}
