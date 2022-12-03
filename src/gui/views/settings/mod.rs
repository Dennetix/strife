mod accounts_tab;
mod appearance_tab;

use iced::{
    alignment::Horizontal,
    widget::{container, row},
    Element, Length,
};
use iced_graphics::Renderer;
use iced_lazy::Component;

use crate::{
    gui::{
        components::sidebar::{sidebar, SidebarEntryType},
        theme::Theme,
    },
    settings::Settings,
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
            SidebarEntryType::Button(Self::Accounts, "Accounts"),
            SidebarEntryType::Button(Self::Appearance, "Appearance"),
        ]
    }
}

#[derive(Debug, Clone)]
pub enum SettingsViewMessage {
    SettingsChanged(Settings),
}

pub fn settings_view<'a, Message>(
    settings: &'a Settings,
    on_message: impl Fn(SettingsViewMessage) -> Message + 'static,
) -> SettingsView<Message> {
    SettingsView::new(settings, on_message)
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
    ThemeSelected(String),
}

pub struct SettingsView<'a, Message> {
    settings: &'a Settings,
    on_message: Box<dyn Fn(SettingsViewMessage) -> Message>,
}

impl<'a, Message> SettingsView<'a, Message> {
    fn new(
        settings: &'a Settings,
        on_message: impl Fn(SettingsViewMessage) -> Message + 'static,
    ) -> Self {
        Self {
            settings,
            on_message: Box::new(on_message),
        }
    }
}

impl<'a, Message, Backend> Component<Message, Renderer<Backend, Theme>>
    for SettingsView<'a, Message>
where
    Backend: iced_graphics::Backend + iced_graphics::backend::Text + 'static,
{
    type State = State;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::TabSelected(SidebarEntryType::Button(tab, _)) => state.active_tab = tab,
            Event::ThemeSelected(id) => {
                let mut settings = self.settings.clone();
                settings.theme = id;
                return Some((self.on_message)(SettingsViewMessage::SettingsChanged(
                    settings,
                )));
            }
            _ => {}
        }

        None
    }

    fn view(&self, state: &Self::State) -> Element<'_, Self::Event, Renderer<Backend, Theme>> {
        let sidebar = sidebar(&Tab::sidebar_entries(), Event::TabSelected);

        let tab = match state.active_tab {
            Tab::Accounts => accounts_tab(),
            Tab::Appearance => appearance_tab(&self.settings.theme),
        };

        row![
            sidebar,
            container(container(tab).max_width(750).width(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .padding([25, 20])
                .align_x(Horizontal::Center)
        ]
        .into()
    }
}

impl<'a, Message, Backend> From<SettingsView<'a, Message>>
    for Element<'a, Message, Renderer<Backend, Theme>>
where
    Message: 'a,
    Backend: iced_graphics::Backend + iced_graphics::backend::Text + 'static,
{
    fn from(settings_view: SettingsView<'a, Message>) -> Self {
        iced_lazy::component(settings_view)
    }
}
