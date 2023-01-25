use iced::widget::svg;
use once_cell::sync::Lazy;

static PATH: Lazy<String> = Lazy::new(|| format!("{}/res/icons", env!("CARGO_MANIFEST_DIR")));

pub static PRIVATE_CHANNELS: Lazy<svg::Handle> =
    Lazy::new(|| svg::Handle::from_path(format!("{}/private_channels.svg", *PATH)));
pub static SETTINGS: Lazy<svg::Handle> =
    Lazy::new(|| svg::Handle::from_path(format!("{}/settings.svg", *PATH)));
pub static USERS: Lazy<svg::Handle> =
    Lazy::new(|| svg::Handle::from_path(format!("{}/users.svg", *PATH)));
pub static X: Lazy<svg::Handle> = Lazy::new(|| svg::Handle::from_path(format!("{}/x.svg", *PATH)));
