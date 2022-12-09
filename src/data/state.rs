use crate::api::gateway::Gateway;

use super::user::User;

#[derive(Debug, Clone)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connecetd(State, Gateway),
}

#[derive(Debug, Clone)]
pub struct State {
    pub user: User,
}
