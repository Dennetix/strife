pub mod components;
pub mod theme;

use iced::{
    executor,
    widget::{container, text},
    Application, Command,
};
use iced_native::row;

use self::{
    components::guildbar::{guildbar, GuildbarEntry},
    theme::{
        data::{DefaultThemes, ThemeData},
        Container, Theme,
    },
};

#[derive(Debug, Clone)]
pub enum Message {
    GuildbarSelect(GuildbarEntry),
}

pub struct App {
    dark: bool,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self { dark: true }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Strife")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::GuildbarSelect(GuildbarEntry::SwitchTheme) => self.dark = !self.dark,
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = container(text("Text"))
            .padding(25)
            .style(Container::BackgroundContrast1);

        container(row![guildbar(Message::GuildbarSelect), content]).into()
    }

    fn theme(&self) -> Self::Theme {
        Theme {
            data: if self.dark {
                ThemeData::dark()
            } else {
                ThemeData::light()
            },
        }
    }
}
