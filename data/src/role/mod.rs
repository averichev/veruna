use std::sync::{Arc};
use async_trait::async_trait;
use serde::Deserialize;
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tokio::sync::Mutex;
use veruna_domain::DataError;
use veruna_domain::roles::{RoleId, RolesRepository as RolesRepositoryTrait};

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}


pub struct RolesRepository {
    connection: Arc<Surreal<Db>>,
}

impl RolesRepository {
    pub(crate) fn new(connection: Arc<Surreal<Db>>) -> Arc<Mutex<RolesRepository>>{
        Arc::new(Mutex::new(RolesRepository{ connection  }))
    }
}

#[async_trait(? Send)]
impl RolesRepositoryTrait for RolesRepository {
    async fn get_role_id(&self, role_name: String) -> Result<Option<RoleId>, Box<dyn DataError>> {
        let mut response = self.connection
            .query("SELECT id FROM roles WHERE code = $role_name")
            .bind(("role_name", role_name))
            .await
            .unwrap();
        let result: Option<Thing> = response.take((0, "id")).unwrap();

        match result {
            None => {
                Ok(None)
            }
            Some(thing) => {
                Ok(Some(RoleId { value: thing.id.to_string() }))
            }
        }
    }
}