use serde::{Deserialize, Serialize};
use crate::users::User;

pub trait UserIdTrait {
    fn value(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserId {
    pub value: String,
}

impl UserIdTrait for UserId{
    fn value(&self) -> String {
        self.value.clone()
    }
}