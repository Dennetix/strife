use iced::widget::svg;
use once_cell::sync::Lazy;

static PATH: Lazy<String> = Lazy::new(|| format!("{}/res/icons", env!("CARGO_MANIFEST_DIR")));

pub static SETTINGS: Lazy<svg::Handle> =
    Lazy::new(|| svg::Handle::from_path(format!("{}/settings.svg", *PATH)));
pub static USER: Lazy<svg::Handle> =
    Lazy::new(|| svg::Handle::from_path(format!("{}/user.svg", *PATH)));
pub static X: Lazy<svg::Handle> = Lazy::new(|| svg::Handle::from_path(format!("{}/x.svg", *PATH)));
