use std::sync::Arc;
use async_trait::async_trait;
use serde::Deserialize;
use surrealdb::engine::local::Db;
use surrealdb::{Error, Surreal};
use surrealdb::sql::Thing;
use veruna_domain::DataError;
use veruna_domain::roles::Role;
use veruna_domain::users::{AddUser, RegisterUser, User, UsersRepository as UsersRepositoryContract};
use veruna_domain::users::user_id::UserId;

#[derive(Debug)]
pub(crate) struct UsersRepository {
    connection: Arc<Surreal<Db>>,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    thing: Thing,
}
#[derive(Debug, Deserialize)]
struct UserEntity {
    #[serde(rename(deserialize = "id"))]
    thing: Thing,
    username: String,
}

impl UsersRepository {
    pub fn new(connection: Arc<Surreal<Db>>) -> Box<dyn UsersRepositoryContract> {
        Box::new(UsersRepository { connection })
    }

}

#[async_trait(? Send)]
impl UsersRepositoryContract for UsersRepository {
    async fn register_user(&mut self, username: String, password: String) -> Result<UserId, Box<dyn DataError>> {
        let record: UserEntity = self.connection
            .create("users")
            .content(RegisterUser {
                username,
                password,
            })
            .await
            .unwrap();
        Ok(UserId { value: record.thing.id.to_string()})
    }

    async fn find_user_id_by_username(&mut self, username: String) -> Option<UserId> {
        let user = self.find_user_by_username(username).await.unwrap();
        match user {
            None => {
                None
            }
            Some(user) => {
                Some(UserId { value: user.id })
            }
        }
    }

    async fn count_users(&mut self) -> u32 {
        let mut response = self.connection
            .query("SELECT VALUE count(id) as count FROM users")
            .await
            .unwrap();
        let count: Option<u32> = response.take(0).unwrap();
        match count {
            None => {
                0
            }
            Some(n) => {
                n
            }
        }
    }

    async fn add_user_role(&mut self, username: String, role: String) {
        todo!()
    }
    async fn find_user_by_username(&self, username: String) -> Result<Option<User>, Box<dyn DataError>> {
        let mut response = self.connection
            .query("SELECT * FROM users WHERE username = $username")
            .bind(("username", username))
            .await
            .unwrap();
        let user: Option<UserEntity> = response.take(0).unwrap();
        match user {
            None => {
                Ok(None)
            }
            Some(user_entity) => {
                Ok(Some(User
                {
                    id: user_entity.thing.id.to_string(),
                    username: user_entity.username,
                })
                )
            }
        }
    }

    async fn add_user(&self, username: String) -> Result<UserId, Box<dyn DataError>> {
        let record: Record = self.connection
            .create("users")
            .content(AddUser {
                username,
            })
            .await
            .unwrap();
        Ok(UserId{ value: record.thing.id.to_string() })
    }
    async fn get_user_roles(&self, username: String) -> Result<Vec<Role>, Box<dyn DataError>> {
        let mut response = self.connection
            .query("SELECT ->has_roles->roles.* as roles FROM type::thing($table, $id);")
            .bind(("table", "roles"))
            .bind(("id", username))
            .await
            .unwrap();
        let roles: Vec<Role> = response.take(0)
            .unwrap();
        Ok(roles)
    }
}