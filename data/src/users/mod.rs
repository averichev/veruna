use std::sync::Arc;
use async_trait::async_trait;
use serde::Deserialize;
use surrealdb::engine::local::Db;
use surrealdb::{Error, Surreal};
use surrealdb::sql::Thing;
use veruna_domain::users::{User, UsersRepository as UsersRepositoryContract};

pub(crate) struct UsersRepository {
    connection: Arc<Surreal<Db>>,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

impl UsersRepository {
    pub fn new(connection: Arc<Surreal<Db>>) -> Box<dyn UsersRepositoryContract> {
        Box::new(UsersRepository { connection })
    }
    async fn find_user_by_username(&self, username: String) -> Result<Option<User>, Error> {
        let mut response = self.connection
            .query("SELECT * FROM users WHERE username = $username")
            .bind(("username", username))
            .await?;
        let sites: Option<User> = response.take(0)?;
        Ok(sites.clone())
    }
    async fn add_user(&self, username: String) -> Result<Thing, Error> {
        let record: Record = self.connection
            .create("users")
            .content(User {
                username,
            })
            .await?;
        Ok(record.id)
    }
}

#[async_trait]
impl UsersRepositoryContract for UsersRepository {
    async fn create_admin(&mut self, username: String) {
        let user = self.find_user_by_username(username.clone()).await.unwrap();
        match user {
            None => {
                println!("Добавляем {} в базу", username);
                let id = self.add_user(username).await.unwrap();
                println!("Новый пользователь создан {}", id.to_string())
            }
            Some(n) => {
                println!("Получаем роли {}", n.username)
            }
        }
    }
}