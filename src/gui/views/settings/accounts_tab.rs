use iced::{
    alignment::Vertical,
    widget::{button, container, horizontal_rule, horizontal_space, svg, text, text_input, Column},
    Element, Length,
};
use iced_graphics::Renderer;
use iced_lazy::Component;
use iced_native::{column, row};

use crate::{
    data::user::User,
    gui::{
        components::{empty, images::user_avatar},
        icons,
        theme::{Button, Text, Theme},
    },
};

fn account_button<'a, Backend>(
    account: &User,
    selected: bool,
) -> Element<'a, Event, Renderer<Backend, Theme>>
where
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Text
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    let content = {
        let remove_button = button(svg(icons::X.clone()))
            .style(Button::TransparentHover(false, Some(12.5)))
            .width(Length::Units(25))
            .height(Length::Units(25))
            .padding(4)
            .on_press(Event::AccountRemoved(account.id.clone()));

        container(row![
            text(&account.username),
            text(format!("#{}", account.discriminator)).style(Text::Weak),
            horizontal_space(Length::Fill),
            remove_button
        ])
        .height(Length::Fill)
        .align_y(Vertical::Center)
    };

    button(row![user_avatar(account, 40), content].spacing(15))
        .style(Button::Border(selected, Some(15.0), 5.0))
        .width(Length::Fill)
        .height(Length::Units(70))
        .padding([15, 20])
        .on_press(Event::AccountSelected(account.id.clone()))
        .into()
}

pub fn accounts_tab<'a, Message>(
    accounts: &'a [User],
    active_account: String,
    on_message: impl Fn(AccountsMessage) -> Message + 'static,
) -> AccountsTab<'a, Message> {
    AccountsTab::new(accounts, active_account, on_message)
}

#[derive(Debug, Clone)]
pub enum AccountsMessage {
    AccountAdded(String),
    AccountSelected(String),
    AccountRemoved(String),
}

#[derive(Default)]
pub struct State {
    token: String,
}

#[derive(Debug, Clone)]
pub enum Event {
    AccountSelected(String),
    AccountRemoved(String),
    TokenChanged(String),
    AddPrssed,
}

pub struct AccountsTab<'a, Message> {
    accounts: &'a [User],
    active_account: String,
    on_message: Box<dyn Fn(AccountsMessage) -> Message>,
}

impl<'a, Message> AccountsTab<'a, Message> {
    fn new(
        accounts: &'a [User],
        active_account: String,
        on_message: impl Fn(AccountsMessage) -> Message + 'static,
    ) -> Self {
        Self {
            accounts,
            active_account,
            on_message: Box::new(on_message),
        }
    }
}

impl<'a, Message, Backend> Component<Message, Renderer<Backend, Theme>> for AccountsTab<'a, Message>
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
            Event::AccountSelected(id) => {
                Some((self.on_message)(AccountsMessage::AccountSelected(id)))
            }
            Event::AccountRemoved(id) => {
                Some((self.on_message)(AccountsMessage::AccountRemoved(id)))
            }
            Event::TokenChanged(token) => {
                state.token = token;
                None
            }
            Event::AddPrssed => {
                let msg = Some((self.on_message)(AccountsMessage::AccountAdded(
                    state.token.clone(),
                )));
                state.token.clear();
                msg
            }
        }
    }

    fn view(&self, state: &Self::State) -> Element<'_, Self::Event, Renderer<Backend, Theme>> {
        let current_account: Element<_, _> =
            if let Some(account) = self.accounts.iter().find(|a| a.id == self.active_account) {
                column![text("Current Account"), account_button(account, true)]
                    .spacing(15)
                    .into()
            } else {
                empty().into()
            };

        let accounts: Element<_, _> = {
            let accounts = self
                .accounts
                .iter()
                .filter(|a| a.id != self.active_account)
                .collect::<Vec<_>>();

            if accounts.len() > 0 {
                column![
                    text("Accounts"),
                    Column::with_children(
                        accounts.iter().map(|a| account_button(a, false)).collect(),
                    )
                    .spacing(5),
                    horizontal_rule(15)
                ]
                .spacing(15)
                .into()
            } else {
                empty().into()
            }
        };

        let add_account = {
            let mut add_button = button("Add").padding([10, 25]);
            if state.token.len() > 60 && state.token.split(".").count() == 3 {
                add_button = add_button.on_press(Event::AddPrssed);
            }

            column![
                text("Add Account"),
                row![
                    text_input("Token", &state.token, Event::TokenChanged)
                        .padding(10)
                        .password(),
                    add_button
                ]
                .spacing(10)
            ]
            .spacing(15)
        };

        column![current_account, accounts, add_account]
            .spacing(15)
            .into()
    }
}

impl<'a, Message, Backend> From<AccountsTab<'a, Message>>
    for Element<'a, Message, Renderer<Backend, Theme>>
where
    Message: 'a,
    Backend: iced_graphics::Backend
        + iced_graphics::backend::Text
        + iced_graphics::backend::Image
        + iced_graphics::backend::Svg
        + 'static,
{
    fn from(account_tab: AccountsTab<'a, Message>) -> Self {
        iced_lazy::component(account_tab)
    }
}
