use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait UsersRepository {
    async fn create_admin(&mut self, username: String);
}
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
}