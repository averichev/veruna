use std::sync::{Arc};
use async_trait::async_trait;
use serde::Deserialize;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use surrealdb::sql::Thing;
use tokio::sync::Mutex;
use veruna_domain::{DataError, RecordId};
use veruna_domain::roles::{Role, RoleId};
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
    password: String,
}

impl UsersRepository {
    pub fn new(connection: Arc<Surreal<Db>>) -> Arc<Mutex<dyn UsersRepositoryContract>> {
        Arc::new(Mutex::new(UsersRepository { connection }))
    }
}

#[async_trait(? Send)]
impl UsersRepositoryContract for UsersRepository {
    async fn register_user(&mut self, user: RegisterUser) -> Result<UserId, Box<dyn DataError>> {
        let record: UserEntity = self.connection
            .create("users")
            .content(user)
            .await
            .unwrap();
        Ok(UserId { value: record.thing.id.to_string() })
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

    async fn count_users(&mut self) -> Result<u32, Box<dyn DataError>> {
        let mut response = self.connection
            .query("SELECT value count FROM (SELECT count() as count FROM users GROUP BY count)")
            .await
            .unwrap();
        let count: Vec<u32> = response.take(0).unwrap();
        let result = count.get(0).unwrap();
        Ok(*result)
    }

    async fn add_user_role(&self, user_id: UserId, role_id: RoleId) -> Result<Option<Box<dyn RecordId>>, Box<dyn DataError>> {
        let mut response = self.connection
            .query("
                LET $user = type::thing($users_table, $user_id);
                LET $role = type::thing($roles_table, $role_id);
                SELECT VALUE id FROM (RELATE $user->has_roles->$role);
            ")
            .bind(("users_table", "users"))
            .bind(("user_id", user_id.value))
            .bind(("roles_table", "roles"))
            .bind(("role_id", role_id.value))
            .await
            .unwrap();
        let result: Option<Thing> = response.take(2).unwrap();
        match result {
            None => {
                Ok(None)
            }
            Some(thing) => {
                Ok(Some(Box::new(crate::RecordId::new(thing.id.to_string()))))
            }
        }
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
                    password: user_entity.password,
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
        Ok(UserId { value: record.thing.id.to_string() })
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