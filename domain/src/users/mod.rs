pub mod user_id;
pub mod errors;
pub mod events;
pub mod models;
pub mod user_list;

use std::sync::Arc;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::{DataErrorTrait, DomainErrorTrait, RecordId};
use crate::roles::{Role, RoleId};
use crate::users::events::{AfterRegisterUserEvent, UserEventsContainer, UsrEvents};
use crate::users::errors::{LoginError, RegisterUserError};
use crate::users::models::claims::{Claims, ClaimsTrait};
use crate::users::user_id::{UserId, UserIdTrait};
use crate::users::user_list::UserListTrait;

#[async_trait(? Send)]
pub trait UsersRepository: Send {
    async fn register_user(&mut self, user: RegisterUser) -> Result<UserId, Box<dyn DataErrorTrait>>;
    async fn find_user_id_by_username(&mut self, username: String) -> Option<UserId>;
    async fn count_users(&mut self) -> Result<u32, Box<dyn DataErrorTrait>>;
    async fn add_user_role(&self, user_id: UserId, role_id: RoleId) -> Result<Option<Box<dyn RecordId>>, Box<dyn DataErrorTrait>>;
    async fn find_user_by_username(&self, username: String) -> Result<Option<User>, Box<dyn DataErrorTrait>>;
    async fn add_user(&self, username: String) -> Result<UserId, Box<dyn DataErrorTrait>>;
    async fn get_user_roles(&self, username: String) -> Result<Vec<Role>, Box<dyn DataErrorTrait>>;
    async fn list(&self) -> Result<Box<dyn UserListTrait>, Box<dyn DataErrorTrait>>;
    async fn delete(&self, user_id: UserId) -> Result<bool, Box<dyn DataErrorTrait>>;
    async fn create(&self, user: Arc<dyn CreateUserTrait>) -> Result<Arc<dyn UserIdTrait>, Box<dyn DataErrorTrait>>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AddUser {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}

pub struct LoginUser {
    pub username: String,
    pub password: String,
}

pub trait CreateUserTrait {
    fn username(&self) ->String;
}

#[async_trait(? Send)]
pub trait UserKitContract {
    async fn register_user(&mut self, username: String, password: String) -> Result<UserId, Box<dyn DomainErrorTrait>>;
    async fn create_admin(&mut self, username: String);
    fn events(self) -> Arc<UserEventsContainer>;
    async fn verify_user_password(&self, user: LoginUser) -> Result<Box<dyn ClaimsTrait>, Box<dyn DomainErrorTrait>>;
    async fn get_user_list(&self) -> Result<Box<dyn UserListTrait>, Box<dyn DomainErrorTrait>>;
    async fn delete_user(&self, user_id: String) -> Result<bool, Box<dyn DomainErrorTrait>>;
    async fn create_user(&self, user: Arc<dyn CreateUserTrait>) -> Result<Arc<dyn UserIdTrait>, Box<dyn DomainErrorTrait>>;
}

pub(crate) struct UserKit {
    pub(crate) repository: Arc<Mutex<dyn UsersRepository>>,
    event_container: Arc<UserEventsContainer>,
}

impl UserKit {
    pub fn new(repository: Arc<Mutex<dyn UsersRepository>>, user_events: Arc<UserEventsContainer>) -> UserKit {
        UserKit { repository, event_container: user_events }
    }
    fn encode_password(&self, plain_password: String, salt: &SaltString) -> String {
        let argon2 = Argon2::default();
        let password = argon2.hash_password(plain_password.as_ref(), salt).unwrap();
        password.to_string()
    }
}

#[async_trait(? Send)]
impl UserKitContract for UserKit {
    async fn register_user(&mut self, username: String, password: String) -> Result<UserId, Box<dyn DomainErrorTrait>> {
        let salt = SaltString::generate(&mut OsRng);

        let encoded_password = self.encode_password(password, &salt);

        let register_result = self.repository.lock().await
            .register_user(RegisterUser {
                username,
                password: encoded_password,
            })
            .await;
        match register_result {
            Ok(user_id) => {
                self.event_container.notify(UsrEvents::AfterRegister(AfterRegisterUserEvent { user_id: user_id.clone() }));
                Ok(user_id)
            }
            Err(data_error) => {
                Err(Box::new(RegisterUserError {
                    message: data_error.message(),
                }))
            }
        }
    }
    async fn create_admin(&mut self, username: String) {
        println!("create_admin");
        let repository = self.repository.lock().await;
        let user = repository.find_user_by_username(username.clone()).await.unwrap();
        match user {
            None => {
                println!("Добавляем {} в базу", username);
                let id = repository.add_user(username).await.unwrap();
                println!("Новый пользователь создан {}", id.value)
            }
            Some(n) => {
                println!("Получаем роли {}", n.username);
                let roles = repository.get_user_roles(n.username).await.unwrap();
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

    async fn verify_user_password(&self, login_user: LoginUser) -> Result<Box<dyn ClaimsTrait>, Box<dyn DomainErrorTrait>> {
        let repository = self.repository.lock().await;
        let user = repository.find_user_by_username(login_user.username).await.unwrap();
        match user {
            None => {
                Err(Box::new(LoginError::new("Пользователь не найден".to_string())))
            }
            Some(user) => {
                let parsed_password = PasswordHash::new(&user.password).unwrap();
                let is_ok = Argon2::default()
                    .verify_password(
                        login_user.password.as_ref(),
                        &parsed_password,
                    )
                    .is_ok();
                if is_ok {
                    Ok(Claims::new(user.username, user.id))
                }
                else {
                    Err(Box::new(LoginError::new("Неверный пароль".to_string())))
                }
            }
        }
    }

    async fn get_user_list(&self) -> Result<Box<dyn UserListTrait>, Box<dyn DomainErrorTrait>> {
        let list = self.repository.lock().await.list().await.unwrap();
        Ok(list)
    }

    async fn delete_user(&self, user_id: String) -> Result<bool, Box<dyn DomainErrorTrait>> {
        let delete = self.repository.lock().await.delete(UserId{ value: user_id }).await.unwrap();
        Ok(delete)
    }

    async fn create_user(&self, user: Arc<dyn CreateUserTrait>) -> Result<Arc<dyn UserIdTrait>, Box<dyn DomainErrorTrait>> {
        let create = self.repository.lock().await.create(user).await.unwrap();
        Ok(create)
    }
}






