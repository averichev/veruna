pub(crate) mod node;
pub(crate) mod site;

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, DbConn, DbErr, NotSet};
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::*, query::*};
use veruna_domain::sites::{Site, SiteBuilder, SiteBuilderImpl, SiteId, SiteIdBuilderImpl, SiteImpl, SiteReadOption, SiteRepository as SiteRepositoryContract};
use entity::site::{ActiveModel, Entity, Model};
use veruna_domain::nodes::NodesRepository;
use std::borrow::Borrow;
use std::fmt::Error;
use surrealdb::{Surreal, engine::local::File};
use surrealdb::engine::any::{Any, connect};
use surrealdb::kvs::Datastore;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::engine::local::Db;
use crate::site::SiteRepositoryImpl;

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
    pub async fn new(connection_string: &str) -> Arc<dyn veruna_domain::input::Repositories> {
        let db: Surreal<Db> = Surreal::new::<File>(connection_string).await.unwrap();
        Arc::new(Repositories{
            connection: Arc::new(db)
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
}