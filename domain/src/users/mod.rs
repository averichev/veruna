pub mod user_id;
pub mod register_user_error;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::DataError;
use crate::users::register_user_error::RegisterUserError;
use crate::users::user_id::UserId;

#[async_trait(? Send)]
pub trait UsersRepository {
    async fn create_admin(&mut self, username: String);
    async fn register_user(&mut self, username: String, password: String) -> Result<UserId, Box<dyn DataError>>;
    async fn find_user_id_by_username(&mut self, username: String) -> Option<UserId>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddUser {
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterUser {
    pub username: String,
    pub password: String
}


#[async_trait(? Send)]
pub trait UserKitContract {
    async fn register_user(&mut self, username: String, password: String) -> Result<UserId, RegisterUserError>;
}

struct UserKit {
    repository: Box<dyn UsersRepository>,
}

#[async_trait(? Send)]
impl UserKitContract for UserKit {
    async fn register_user(&mut self, username: String, password: String) -> Result<UserId, RegisterUserError> {
        let register_result = self.repository.register_user(username, password).await;
        match register_result {
            Ok(user_id) => {
                Ok(user_id)
            }
            Err(data_error) => {
                Err(RegisterUserError {
                    message: data_error.message(),
                })
            }
        }
    }
}