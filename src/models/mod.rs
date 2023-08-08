use std::rc::Rc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) username: String,
    pub(crate) id: String,
}

pub(crate) trait CurrentUserTrait {
    fn username(&self) -> String;
}

#[derive(Clone)]
pub(crate) struct CurrentUser {
    username: Option<String>,
}

impl CurrentUser {
    pub(crate) fn new() -> CurrentUser {
        CurrentUser { username: None }
    }
}

impl CurrentUserTrait for CurrentUser {
    fn username(&self) -> String {
        (&self.username.clone().unwrap()).to_string()
    }
}
