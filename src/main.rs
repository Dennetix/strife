mod gui;

use iced::{Application, Settings};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use gui::App;

#[tokio::main]
async fn main() -> iced::Result {
    // Logging
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish(),
    )
    .expect("Failed to set default subscriber");

    App::run(Settings::default())
}
