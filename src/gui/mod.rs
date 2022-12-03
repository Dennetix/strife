pub mod components;
pub mod message;
pub mod theme;
pub mod views;

use std::sync::Arc;

use iced::{executor, widget::text, Application, Command, Element, Renderer};
use iced_native::row;
use tracing::error;

use crate::settings::Settings;

use self::{
    components::guildbar::{guildbar, View},
    message::Message,
    theme::{
        data::{DefaultThemes, ThemeData},
        Theme,
    },
    views::settings::{settings_view, SettingsViewMessage},
};

pub fn map_result_message<T, Message>(
    f: impl FnOnce(Result<T, Arc<anyhow::Error>>) -> Message + 'static,
) -> impl FnOnce(anyhow::Result<T>) -> Message + 'static {
    |r| match r {
        Ok(t) => f(Ok(t)),
        Err(e) => f(Err(Arc::new(e))),
    }
}

pub struct App {
    settings: Settings,
    active_view: View,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                settings: Settings::default(),
                active_view: View::Settings,
            },
            Command::perform(Settings::load(), Message::SettingsLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("Strife")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SettingsLoaded(settings) => self.settings = settings,
            Message::SettingsSaved(res) => {
                if let Err(e) = res {
                    error!("Failed to save settings, {e}");
                }
            }
            Message::ViewSelect(view) => self.active_view = view,

            Message::SettingsViewMessage(message) => match message {
                SettingsViewMessage::SettingsChanged(settings) => {
                    self.settings = settings.clone();
                    return Command::perform(
                        settings.save(),
                        map_result_message(Message::SettingsSaved),
                    );
                }
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let view: Element<'_, Self::Message, Renderer<Self::Theme>> = match self.active_view {
            View::PrivateChannels => text("Private Channels").into(),
            View::Settings => settings_view(&self.settings, Message::SettingsViewMessage).into(),
        };

        row![
            guildbar(self.active_view.clone(), Message::ViewSelect),
            view
        ]
        .into()
    }

    fn theme(&self) -> Self::Theme {
        let data = match self.settings.theme.as_str() {
            "strife.light" => ThemeData::light(),
            "strife.dark" => ThemeData::dark(),
            _ => ThemeData::default(),
        };

        Theme { data }
    }
}
