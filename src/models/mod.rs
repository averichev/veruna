use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) username: String,
    pub(crate) id: String,
}

pub(crate) trait CurrentUserTrait: Send + Sync + 'static {
    fn username(&self) -> String;
    fn set_user_name(&mut self, username: String);
    fn set_user_id(&mut self, user_id: String);
    fn id(&self) -> String;
}

#[derive(Clone)]
pub(crate) struct CurrentUser  {
    username: Option<String>,
    id: Option<String>,
}

impl CurrentUser {
    pub(crate) fn new() -> CurrentUser {
        CurrentUser { username: None, id: None }
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

    fn set_user_id(&mut self, user_id: String) {
        self.id = Some(user_id);
    }

    fn id(&self) -> String {
        self.id.clone().unwrap()
    }
}
