pub mod theme;

mod components;
mod icons;
mod message;
mod views;

use iced::{executor, widget::text, Application, Command, Element, Renderer};
use iced_native::row;
use tracing::error;

use crate::{
    api::{cdn_client::CdnClient, gateway::Gateway},
    data::{settings::Settings, state::ConnectionState, user::User},
};

use self::{
    components::guildbar::{guildbar, View},
    message::{map_result_message, Message},
    theme::{
        data::{DefaultThemes, ThemeData},
        Theme,
    },
    views::settings::{settings_view, AccountsMessage, SettingsViewMessage},
};

const SERVICE: &str = "strife_accounts";

pub struct App {
    connection_state: ConnectionState,
    active_view: View,
    settings: Settings,
    accounts: Vec<User>,
    cdn_client: CdnClient,
}

impl App {
    fn save_settings(&self) -> Command<Message> {
        Command::perform(
            self.settings.clone().save(),
            map_result_message(Message::SettingsSaved),
        )
    }
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                connection_state: ConnectionState::Disconnected,
                active_view: View::Settings,
                settings: Settings::default(),
                accounts: vec![],
                cdn_client: CdnClient::new(),
            },
            Command::perform(Settings::load(), Message::SettingsLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("Strife")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SettingsLoaded(settings) => {
                self.settings = settings;
                let accounts = self.settings.accounts.drain(..);

                // Load all connected accounts
                let mut commands = accounts
                    .filter_map(|id| keyring::Entry::new(SERVICE, &id).get_password().ok())
                    .map(|token| {
                        Command::perform(
                            User::from_token(token),
                            map_result_message(|user| Message::AccountLoaded(user, None)),
                        )
                    })
                    .collect::<Vec<_>>();

                // Connect the gateway to the last active account
                if self.settings.active_account.len() > 0 {
                    if let Ok(token) =
                        keyring::Entry::new(SERVICE, &self.settings.active_account).get_password()
                    {
                        self.connection_state = ConnectionState::Connecting;

                        commands.push(Command::perform(
                            Gateway::new(token),
                            map_result_message(Message::Connected),
                        ));
                    } else {
                        self.connection_state = ConnectionState::Disconnected;
                        error!("Keyring did not contain the token of the selected account");
                    }
                }

                return Command::batch(commands);
            }
            Message::SettingsSaved(res) => {
                if let Err(e) = res {
                    error!("Failed to save settings, {e}");
                }
            }
            Message::AccountLoaded(user, token) => match user {
                Ok(user) => {
                    let (id, avatar) = (user.id.clone(), user.avatar.clone());

                    if let None = self.settings.accounts.iter().find(|a| **a == id) {
                        if let Some(token) = token {
                            if let Err(e) = keyring::Entry::new(SERVICE, &id).set_password(&token) {
                                error!("Failed to save account token to keyring: {e}");
                            }
                        }

                        self.accounts.push(user);
                        self.settings.accounts.push(id.clone());

                        return Command::batch([
                            self.save_settings(),
                            if let Some(avatar) = avatar {
                                Command::perform(
                                    self.cdn_client.clone().avatar(id.clone(), avatar, 128),
                                    map_result_message(|handle| {
                                        Message::AccountAvatarLoaded(id, handle)
                                    }),
                                )
                            } else {
                                Command::none()
                            },
                        ]);
                    }
                }
                Err(e) => error!("Failed to load account: {e}"),
            },
            Message::AccountAvatarLoaded(id, handle) => {
                if let Some(user) = self.accounts.iter_mut().find(|u| u.id == id) {
                    match handle {
                        Ok(handle) => user.avatar_handle = Some(handle),
                        Err(e) => error!("Failed to load user avatar: {e}"),
                    }
                }
            }
            Message::Connected(res) => match res {
                Ok((gateway, state)) => {
                    self.connection_state = ConnectionState::Connecetd(state, gateway)
                }
                Err(e) => {
                    self.connection_state = ConnectionState::Disconnected;
                    error!("Failed to connect to gateway: {e}");
                }
            },

            Message::ViewSelect(view) => self.active_view = view,

            Message::SettingsViewMessage(message) => match message {
                SettingsViewMessage::SettingsChanged(settings) => {
                    self.settings = settings;
                    return self.save_settings();
                }
                SettingsViewMessage::AccountsMessage(message) => match message {
                    AccountsMessage::AccountAdded(token) => {
                        return Command::perform(
                            User::from_token(token.clone()),
                            map_result_message(|user| Message::AccountLoaded(user, Some(token))),
                        )
                    }
                    AccountsMessage::AccountSelected(id) => {
                        if let ConnectionState::Connecetd(_, gateway) = &mut self.connection_state {
                            gateway.close();
                        }

                        if let Ok(token) = keyring::Entry::new(SERVICE, &id).get_password() {
                            self.settings.active_account = id;
                            self.connection_state = ConnectionState::Connecting;

                            return Command::batch([
                                self.save_settings(),
                                Command::perform(
                                    Gateway::new(token),
                                    map_result_message(Message::Connected),
                                ),
                            ]);
                        } else {
                            self.connection_state = ConnectionState::Disconnected;
                            error!("Keyring did not contain the token of the selected account");
                        }
                    }
                    AccountsMessage::AccountRemoved(id) => {
                        if self.settings.active_account == id {
                            self.settings.active_account.clear();

                            if let ConnectionState::Connecetd(_, gateway) =
                                &mut self.connection_state
                            {
                                gateway.close();
                                self.connection_state = ConnectionState::Disconnected;
                            }
                        }
                        self.settings.accounts.retain(|a| *a != id);
                        self.accounts.retain(|a| a.id != id);
                        return self.save_settings();
                    }
                },
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let view: Element<'_, Self::Message, Renderer<Self::Theme>> = match self.active_view {
            View::PrivateChannels => text("Private Channels").into(),
            View::Settings => {
                settings_view(&self.settings, &self.accounts, Message::SettingsViewMessage).into()
            }
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
