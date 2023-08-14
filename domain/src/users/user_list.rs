use std::sync::Arc;

pub trait UserListItemTrait {
    fn username(&self) -> String;
    fn id(&self) -> String;
}

#[derive(Clone)]
pub struct UserListItem {
    username: String,
    id: String
}

impl UserListItem {
    pub fn new(id: String, username: String) -> Box<dyn UserListItemTrait> {
        Box::new(UserListItem { username, id })
    }
}

impl UserListItemTrait for UserListItem {
    fn username(&self) -> String {
        self.username.clone()
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

pub trait UserListTrait {
    fn list(&self) -> Arc<Vec<Box<dyn UserListItemTrait>>>;
}

pub struct UserList {
    list: Arc<Vec<Box<dyn UserListItemTrait>>>,
}

impl UserList{
    pub fn new(list: Vec<Box<dyn UserListItemTrait>>) -> Box<dyn UserListTrait> {
        Box::new(UserList{ list: Arc::new(list) })
    }
}

impl UserListTrait for UserList {
    fn list(&self) -> Arc<Vec<Box<dyn UserListItemTrait>>> {
        self.list.clone()
    }
}
