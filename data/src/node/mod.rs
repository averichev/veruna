use std::sync::Arc;
use async_trait::async_trait;
use surrealdb::Surreal;
use veruna_domain::nodes::{Node, NodesRepository};
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