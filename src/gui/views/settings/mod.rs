mod accounts_tab;
mod appearance_tab;

pub use self::accounts_tab::AccountsMessage;

use iced::widget::scrollable::Properties;
use iced::{
    alignment::Horizontal,
    widget::{container, row, scrollable},
    Element, Length,
};
use iced_graphics::Renderer;
use iced_lazy::{lazy, Component};

use crate::{
    data::{settings::Settings, user::User},
    gui::{
        components::sidebar::{sidebar, SidebarEntryType},
        theme::Theme,
    },
};

use self::{accounts_tab::accounts_tab, appearance_tab::appearance_tab};

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Accounts,
    Appearance,
}

impl Tab {
    fn sidebar_entries() -> Vec<SidebarEntryType<Self>> {
        vec![
            SidebarEntryType::Button(Self::Accounts, String::from("Accounts")),
            SidebarEntryType::Button(Self::Appearance, String::from("Appearance")),
        ]
    }
}

pub fn settings_view<'a, Message>(
    settings: &'a Settings,
    accounts: &'a [User],
    on_message: impl Fn(SettingsViewMessage) -> Message + 'static,
) -> SettingsView<'a, Message> {
    SettingsView::new(settings, accounts, on_message)
}

#[derive(Debug, Clone)]
pub enum SettingsViewMessage {
    SettingsChanged(Settings),
    AccountsMessage(AccountsMessage),
}

pub struct State {
    active_tab: Tab,
}

impl Default for State {
    fn default() -> Self {
        State {
            active_tab: Tab::Accounts,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    TabSelected(SidebarEntryType<Tab>),
    AccountsMessage(AccountsMessage),
    ThemeSelected(String),
}

pub struct SettingsView<'a, Message> {
    settings: &'a Settings,
    accounts: &'a [User],
    on_message: Box<dyn Fn(SettingsViewMessage) -> Message>,
}

impl<'a, Message> SettingsView<'a, Message> {
    fn new(
        settings: &'a Settings,
        accounts: &'a [User],
        on_message: impl Fn(SettingsViewMessage) -> Message + 'static,
    ) -> Self {
        Self {
            settings,
            accounts,
            on_message: Box::new(on_message),
        }
    }
}

impl<'a, Message, Backend> Component<Message, Renderer<Backend, Theme>>
    for SettingsView<'a, Message>
where
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Text
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    type State = State;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::TabSelected(SidebarEntryType::Button(tab, _)) => {
                state.active_tab = tab;
                None
            }
            Event::AccountsMessage(message) => Some((self.on_message)(
                SettingsViewMessage::AccountsMessage(message),
            )),
            Event::ThemeSelected(id) => {
                let mut settings = self.settings.clone();
                settings.theme = id;
                Some((self.on_message)(SettingsViewMessage::SettingsChanged(
                    settings,
                )))
            }
            _ => None,
        }
    }

    fn view(&self, state: &Self::State) -> Element<'_, Self::Event, Renderer<Backend, Theme>> {
        let sidebar = lazy((), |_| sidebar(&Tab::sidebar_entries(), Event::TabSelected));

        let tab: Element<_, _> = match state.active_tab {
            Tab::Accounts => accounts_tab(
                self.accounts,
                self.settings.active_account.clone(),
                Event::AccountsMessage,
            )
            .into(),
            Tab::Appearance => appearance_tab(&self.settings.theme),
        };

        let content = container(
            scrollable(
                container(container(tab).max_width(750).width(Length::Fill))
                    .width(Length::Fill)
                    .padding([25, 20])
                    .align_x(Horizontal::Center),
            )
            .height(Length::Fill)
            .vertical_scroll(Properties::new().width(5).scroller_width(5).margin(8)),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        row![sidebar, content].into()
    }
}

impl<'a, Message, Backend> From<SettingsView<'a, Message>>
    for Element<'a, Message, Renderer<Backend, Theme>>
where
    Message: 'a,
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Text
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    fn from(settings_view: SettingsView<'a, Message>) -> Self {
        iced_lazy::component(settings_view)
    }
}
