use serde::{Deserialize, Serialize};
use crate::users::events::UserEventsContainer;

#[derive(Serialize, Deserialize, Clone)]
pub struct Role {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RoleId {
    pub value: String,
}

pub struct UserEventsListener;

impl UserEventsListener {
    pub(crate) fn new(user_events: UserEventsContainer) {

    }
}