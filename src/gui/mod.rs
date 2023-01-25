pub mod theme;

mod components;
mod icons;
mod message;
mod views;

use iced::{executor, widget::text, Application, Command, Element, Renderer, Subscription};
use iced_native::row;
use tracing::error;

use crate::{
    api::{
        cdn_client::CdnClient,
        gateway::{Gateway, GatewayEvent},
    },
    data::{
        settings::Settings,
        state::{ConnectionState, Message},
        user::User,
    },
};

use self::{
    components::guildbar::{guildbar, View},
    message::{map_result_message, AppMessage},
    theme::{
        data::{DefaultThemes, ThemeData},
        Theme,
    },
    views::{
        direct_messages::direct_messages_view,
        settings::{settings_view, AccountsMessage, SettingsViewMessage},
    },
};

const SERVICE: &str = "strife_accounts";

pub struct App {
    connection_state: ConnectionState,
    settings: Settings,
    active_view: View,
    accounts: Vec<User>,
    cdn_client: CdnClient,
}

impl App {
    fn save_settings(&self) -> Command<AppMessage> {
        Command::perform(
            self.settings.clone().save(),
            map_result_message(AppMessage::SettingsSaved),
        )
    }
}

impl Application for App {
    type Message = AppMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                connection_state: ConnectionState::Disconnected,
                settings: Settings::default(),
                active_view: View::Settings,
                accounts: vec![],
                cdn_client: CdnClient::new(),
            },
            Command::perform(Settings::load(), AppMessage::SettingsLoaded),
        )
    }

    fn title(&self) -> String {
        String::from("Strife")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::SettingsLoaded(settings) => {
                self.settings = settings;
                let accounts = self.settings.accounts.drain(..);

                // Load all connected accounts
                let mut commands = accounts
                    .filter_map(|id| keyring::Entry::new(SERVICE, &id).get_password().ok())
                    .map(|token| {
                        Command::perform(
                            User::from_token(token),
                            map_result_message(|user| AppMessage::AccountLoaded(user, None)),
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
                            map_result_message(AppMessage::GatewayConnected),
                        ));
                    } else {
                        self.connection_state = ConnectionState::Disconnected;
                        error!("Keyring did not contain the token of the selected account");
                    }
                }

                return Command::batch(commands);
            }
            AppMessage::SettingsSaved(res) => {
                if let Err(e) = res {
                    error!("Failed to save settings, {e}");
                }
            }
            AppMessage::AccountLoaded(user, token) => match user {
                Ok(user) => {
                    let (id, discriminator, avatar) = (
                        user.id.clone(),
                        user.discriminator.clone(),
                        user.avatar.clone(),
                    );

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
                                    self.cdn_client.clone().avatar(id.clone(), avatar, 64),
                                    map_result_message(|handle| {
                                        AppMessage::AccountAvatarLoaded(id, handle)
                                    }),
                                )
                            } else {
                                Command::perform(
                                    self.cdn_client.clone().default_avatar(discriminator),
                                    map_result_message(|handle| {
                                        AppMessage::AccountAvatarLoaded(id, handle)
                                    }),
                                )
                            },
                        ]);
                    }
                }
                Err(e) => error!("Failed to load account: {e}"),
            },
            AppMessage::AccountAvatarLoaded(id, handle) => {
                if let Some(user) = self.accounts.iter_mut().find(|u| u.id == id) {
                    match handle {
                        Ok(handle) => user.avatar_handle = Some(handle),
                        Err(e) => error!("Failed to load user avatar: {e}"),
                    }
                }
            }
            AppMessage::GatewayConnected(res) => match res {
                Ok((gateway, state)) => {
                    self.connection_state = ConnectionState::Connecetd(state.clone(), gateway);

                    // Create commands to load user avatars
                    let user_commands = state.user_cache.into_iter().map(|(_, user)| {
                        if let Some(avatar) = user.avatar {
                            Command::perform(
                                self.cdn_client.clone().avatar(user.id.clone(), avatar, 64),
                                map_result_message(|handle| {
                                    AppMessage::UserAvatarLoaded(user.id, handle)
                                }),
                            )
                        } else {
                            Command::perform(
                                self.cdn_client.clone().default_avatar(user.discriminator),
                                map_result_message(|handle| {
                                    AppMessage::UserAvatarLoaded(user.id, handle)
                                }),
                            )
                        }
                    });

                    // Create commands to load group icons
                    let group_commands = state.private_channels.into_iter().flat_map(|c| {
                        if let Some(icon) = c.icon {
                            Some(Command::perform(
                                self.cdn_client.clone().channel_icon(c.id.clone(), icon, 64),
                                map_result_message(|handle| {
                                    AppMessage::GroupIconLoaded(c.id, handle)
                                }),
                            ))
                        } else {
                            None
                        }
                    });

                    return Command::batch(user_commands.chain(group_commands));
                }
                Err(e) => {
                    self.connection_state = ConnectionState::Disconnected;
                    error!("Failed to connect to gateway: {e}");
                }
            },

            AppMessage::GatewayEvent(event) => match event {
                GatewayEvent::ReconnectNeeded => {
                    if let Ok(token) =
                        keyring::Entry::new(SERVICE, &self.settings.active_account).get_password()
                    {
                        self.connection_state = ConnectionState::Connecting;
                        return Command::perform(
                            Gateway::new(token),
                            map_result_message(AppMessage::GatewayConnected),
                        );
                    } else {
                        self.connection_state = ConnectionState::Disconnected;
                        error!("Keyring did not contain the token of the selected account");
                    }
                }
                GatewayEvent::Message(msg) => {
                    if let ConnectionState::Connecetd(state, _) = &mut self.connection_state {
                        state.insert_message(
                            msg.channel_id,
                            Message::Default {
                                user_id: msg.author.id,
                                content: msg.content,
                            },
                        )
                    }
                }
            },

            AppMessage::UserAvatarLoaded(id, handle) => match handle {
                Ok(handle) => {
                    if let ConnectionState::Connecetd(state, _) = &mut self.connection_state {
                        state
                            .user_cache
                            .entry(id)
                            .and_modify(|u| u.avatar_handle = Some(handle));
                    }
                }
                Err(e) => error!("Failed to load user avatar: {e}"),
            },
            AppMessage::GroupIconLoaded(id, handle) => match handle {
                Ok(handle) => {
                    if let ConnectionState::Connecetd(state, _) = &mut self.connection_state {
                        if let Some(channel) =
                            state.private_channels.iter_mut().find(|c| c.id == id)
                        {
                            channel.icon_handle = Some(handle);
                        }
                    }
                }
                Err(e) => error!("Failed to load group icon: {e}"),
            },

            AppMessage::ViewSelect(view) => {
                if let ConnectionState::Connecetd(_, _) = self.connection_state {
                    self.active_view = view;
                } else {
                    self.active_view = View::Settings;
                }
            }

            AppMessage::SettingsViewMessage(message) => match message {
                SettingsViewMessage::SettingsChanged(settings) => {
                    self.settings = settings;
                    return self.save_settings();
                }
                SettingsViewMessage::AccountsMessage(message) => match message {
                    AccountsMessage::AccountAdded(token) => {
                        return Command::perform(
                            User::from_token(token.clone()),
                            map_result_message(|user| AppMessage::AccountLoaded(user, Some(token))),
                        )
                    }
                    AccountsMessage::AccountSelected(id) => {
                        if id != self.settings.active_account {
                            if let ConnectionState::Connecetd(_, gateway) =
                                &mut self.connection_state
                            {
                                gateway.close();
                            }

                            if let Ok(token) = keyring::Entry::new(SERVICE, &id).get_password() {
                                self.settings.active_account = id;
                                self.connection_state = ConnectionState::Connecting;

                                return Command::batch([
                                    self.save_settings(),
                                    Command::perform(
                                        Gateway::new(token),
                                        map_result_message(AppMessage::GatewayConnected),
                                    ),
                                ]);
                            } else {
                                self.connection_state = ConnectionState::Disconnected;
                                error!("Keyring did not contain the token of the selected account");
                            }
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
            AppMessage::DirectMessagesViewMessage(_) => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        if let ConnectionState::Connecetd(_, gateway) = &self.connection_state {
            gateway.subscribe().map(AppMessage::GatewayEvent)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        let view: Element<'_, Self::Message, Renderer<Self::Theme>> = match self.active_view {
            View::DirectMessages => {
                if let ConnectionState::Connecetd(state, _) = &self.connection_state {
                    direct_messages_view(state, AppMessage::DirectMessagesViewMessage).into()
                } else {
                    text("This should never be seen").into()
                }
            }
            View::Settings => settings_view(
                &self.settings,
                &self.accounts,
                AppMessage::SettingsViewMessage,
            )
            .into(),
        };

        row![
            guildbar(self.active_view.clone(), AppMessage::ViewSelect),
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
