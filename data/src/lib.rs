pub(crate) mod node;
pub(crate) mod site;
mod users;
mod role;

use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection};
use sea_orm::{entity::*, query::*};
use veruna_domain::sites::{Site, SiteBuilder, SiteId, SiteRepository as SiteRepositoryContract};
use veruna_domain::nodes::NodesRepository;
use std::borrow::Borrow;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use crate::site::SiteRepositoryImpl;
use crate::users::UsersRepository;

pub struct ConnectionBuilder {
    connection: Arc<DatabaseConnection>,
}

impl ConnectionBuilder {
    pub async fn new(database_url: String) -> ConnectionBuilder {
        let connection = Arc::new(Database::connect(database_url)
            .await
            .expect("Failed to setup the database"));
        ConnectionBuilder {
            connection
        }
    }

}

pub struct Repositories {
    connection: Arc<Surreal<Db>>,
}

impl Repositories {
    pub async fn new(connection: Arc<Surreal<Db>>) -> Arc<dyn veruna_domain::input::Repositories> {
        Arc::new(Repositories{
            connection
        })
    }
}

#[async_trait(? Send)]
impl veruna_domain::input::Repositories for Repositories {
    async fn site(&self) -> Box<dyn SiteRepositoryContract> {
        SiteRepositoryImpl::new(self.connection.clone()).await
    }

    async fn nodes(&self) -> Box<dyn NodesRepository> {
        node::NodesRepositoryImpl::new(self.connection.clone()).await
    }

    async fn users(&self) -> Box<dyn veruna_domain::users::UsersRepository> {
        UsersRepository::new(self.connection.clone())
    }
}