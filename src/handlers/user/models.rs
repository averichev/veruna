use linq::iter::Enumerable;
use serde::{Deserialize, Serialize};
use validator::Validate;
use veruna_domain::users::CreateUserTrait;
use veruna_domain::users::user_list::{UserListItemTrait, UserListTrait};

#[derive(Serialize)]
struct UserListItem {
    username: String,
    id: String,
}

#[derive(Serialize)]
pub(crate) struct UserList {
    items: Vec<UserListItem>,
}

impl UserList {
    pub(crate) fn new(list: Box<dyn UserListTrait>) -> UserList {
        let items = list.list().iter()
            .select(|n| UserListItem { username: n.username(), id: n.id() })
            .collect();
        UserList { items }
    }
}

#[derive(Deserialize)]
pub(crate) struct DeleteUserRequest {
    pub(crate) user_id: String,
}

#[derive(Deserialize, Validate)]
pub(crate) struct CreateUserRequest {
    #[validate(length(min = 3, message = "Должно содержать минимум 3 символа"))]
    pub(crate) username: String,
}

impl CreateUserTrait for CreateUserRequest {
    fn username(&self) -> String {
        self.username.clone()
    }
}

#[derive(Serialize)]
pub(crate) struct UserId {
    pub(crate) value: String,
}

#[derive(Serialize)]
pub(crate) struct CreateUserResponse {
    pub(crate) user_id: UserId,
}