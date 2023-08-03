pub mod user_id;
pub mod register_user_error;
pub mod events;

use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crossbeam_channel::SendError;
use serde::{Deserialize, Serialize};
use crate::{DataError, DomainError};
use crate::roles::{Role, RoleId};
use crate::users::events::{AfterRegisterUserEvent, UserEventsContainer, UsrEvents};
use crate::users::register_user_error::RegisterUserError;
use crate::users::user_id::UserId;

#[async_trait(? Send)]
pub trait UsersRepository {
    async fn register_user(&mut self, username: String, password: String) -> Result<UserId, Box<dyn DataError>>;
    async fn find_user_id_by_username(&mut self, username: String) -> Option<UserId>;
    async fn count_users(&mut self) -> u32;
    async fn add_user_role(self, user_id: UserId, role_id: RoleId);
    async fn find_user_by_username(&self, username: String) -> Result<Option<User>, Box<dyn DataError>>;
    async fn add_user(&self, username: String) -> Result<UserId, Box<dyn DataError>>;
    async fn get_user_roles(&self, username: String) -> Result<Vec<Role>, Box<dyn DataError>>;
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
    fn events(self) -> Arc<UserEventsContainer>;
}

pub(crate) struct UserKit {
    pub(crate) repository: Box<dyn UsersRepository>,
    event_container: Arc<UserEventsContainer>,
}

impl UserKit {
    pub fn new(repository: Box<dyn UsersRepository>, user_events: Arc<UserEventsContainer>) -> UserKit {
        UserKit { repository, event_container: user_events }
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
                self.event_container.notify(UsrEvents::AfterRegister(AfterRegisterUserEvent { user_id: user_id.clone() }));
                // println!("user exist");
                // let count_users: u32 = self.repository.count_users().await;
                // println!("create admin because 1 user only");
                // if count_users == 1u32 {
                //     self.create_admin(username).await;
                // }

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
                let id = self.repository.add_user(username).await.unwrap();
                println!("Новый пользователь создан {}", id.value)
            }
            Some(n) => {
                println!("Получаем роли {}", n.username);
                let roles = self.repository.get_user_roles(n.username).await.unwrap();
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

    fn events(self) -> Arc<UserEventsContainer> {
        self.event_container.clone()
    }
}