use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) username: String,
    pub(crate) id: String,
}

pub(crate) trait CurrentUserTrait: Send + Sync + 'static {
    fn username(&self) -> String;
    fn set_user_name(&mut self, username: String);
}

#[derive(Clone)]
pub(crate) struct CurrentUser  {
    username: Option<String>,
}

impl CurrentUser {
    pub(crate) fn new() -> CurrentUser {
        CurrentUser { username: None }
    }
}

impl CurrentUserTrait for CurrentUser {
    fn username(&self) -> String {
        let result = (&self.username.clone().unwrap()).to_string();
        println!("get username, {}", result);
        result
    }

    fn set_user_name(&mut self, username: String) {
        println!("set_user_name, {}", username);
        self.username = Some(username);
    }
}
