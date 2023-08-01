pub mod user_id;
pub mod register_user_error;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::{DataError, DomainError};
use crate::roles::Role;
use crate::users::register_user_error::RegisterUserError;
use crate::users::user_id::UserId;

#[async_trait]
pub trait UsersRepository {
    async fn register_user(&mut self, username: String, password: String) -> Result<UserId, Box<dyn DataError>>;
    async fn find_user_id_by_username(&mut self, username: String) -> Option<UserId>;
    async fn count_users(&mut self) -> u32;
    async fn add_user_role(&mut self, username: String, role: String);
    async fn find_user_by_username(&self, username: String) -> Result<Option<User>, Error>;
    async fn add_user(&self, username: String) -> Result<Thing, Error>;
    async fn get_user_roles(&self, username: String) -> Result<Vec<Role>, Error>;
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
    pub password: String,
}


#[async_trait(? Send)]
pub trait UserKitContract {
    async fn register_user(&mut self, username: String, password: String) -> Result<UserId, Box<dyn DomainError>>;
    async fn create_admin(&mut self, username: String);
}

pub(crate) struct UserKit {
    pub(crate) repository: Box<dyn UsersRepository>,
}

impl UserKit {
    pub fn new(repository: Box<dyn UsersRepository>) -> UserKit {
        UserKit { repository }
    }
}

#[async_trait(? Send)]
impl UserKitContract for UserKit {
    async fn register_user(&mut self, username: String, password: String) -> Result<UserId, Box<dyn DomainError>> {
        println!("register_user domain");
        let register_result = self.repository
            .register_user(username.clone(), password)
            .await;
        match register_result {
            Ok(user_id) => {
                println!("user exist");
                let count_users: u32 = self.repository.count_users().await;
                println!("create admin because 1 user only");
                if count_users == 1u32 {
                    self.repository.create_admin(username).await;
                }
                Ok(user_id)
            }
            Err(data_error) => {
                println!("error");
                Err(Box::new(RegisterUserError {
                    message: data_error.message(),
                }))
            }
        }
    }
    async fn create_admin(&mut self, username: String) {
        println!("create_admin");
        let user = self.repository.find_user_by_username(username.clone()).await.unwrap();
        match user {
            None => {
                println!("Добавляем {} в базу", username);
                let id = self.add_user(username).await.unwrap();
                println!("Новый пользователь создан {}", id.to_string())
            }
            Some(n) => {
                println!("Получаем роли {}", n.username);
                let roles = self.get_user_roles(n.username).await.unwrap();
                if roles.is_empty() {
                    println!("Роли отсутствуют, добавляем");
                } else {
                    println!("Роли получены:");
                    for role in roles {
                        println!("- {}", role.name);
                    }
                }
            }
        }
    }
}