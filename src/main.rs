mod api;
mod data;
mod gui;

use anyhow::{Context, Result};
use iced::{window, Application, Settings};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use gui::App;

#[tokio::main]
async fn main() -> Result<()> {
    // Logging
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish(),
    )
    .context("Failed to set default subscriber")?;

    App::run(Settings {
        window: window::Settings {
            min_size: Some((950, 600)),
            ..Default::default()
        },
        ..Default::default()
    })?;

    Ok(())
}
