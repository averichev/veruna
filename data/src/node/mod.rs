use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{Database, DatabaseConnection, DbConn, DbErr};
use entity::nodes::{Entity, Model};
use sea_orm::{entity::*, query::*};
use surrealdb::engine::any::Any;
use surrealdb::kvs::Datastore;
use surrealdb::Surreal;
use entity::prelude::Nodes;
use veruna_domain::nodes::{Node, NodeModel, NodesRepository};
use surrealdb::engine::local::Db;

pub(crate) struct NodesRepositoryImpl {
    connection: Arc<Surreal<Db>>,
}

impl NodesRepositoryImpl {
    pub(crate) async fn new(connection: Arc<Surreal<Db>>) -> Box<dyn NodesRepository> {
        let result = NodesRepositoryImpl { connection };
        Box::new(result)
    }
}

#[async_trait(? Send)]
impl NodesRepository for NodesRepositoryImpl {
    async fn find_path(&self, path: String) -> Option<Box<dyn Node>> {
        None
    }
}