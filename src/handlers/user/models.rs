use linq::iter::Enumerable;
use serde::{Deserialize, Serialize};
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