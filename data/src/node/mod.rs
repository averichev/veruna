use std::ops::Deref;
use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{Database, DatabaseConnection, DbConn, DbErr};
use entity::nodes::{Entity, Model};
use sea_orm::{entity::*, query::*};
use entity::prelude::Nodes;
use veruna_domain::nodes::{Node, NodeModel, NodesRepository};

pub(crate) struct NodesRepositoryImpl {
    connection: Arc<DatabaseConnection>,
}

impl NodesRepositoryImpl {
    pub(crate) async fn new(connection: Arc<DatabaseConnection>) -> Box<dyn NodesRepository> {
        let result = NodesRepositoryImpl { connection };
        Box::new(result)
    }
}

#[async_trait(? Send)]
impl NodesRepository for NodesRepositoryImpl {
    async fn find_path(&self, path: String) -> Option<Box<dyn Node>> {
        let entity = Entity::find()
            .filter(<Nodes as EntityTrait>::Column::Path.eq(path))
            .one(self.connection.deref())
            .await
            .unwrap();
        match entity {
            Some(e) => {
                let result = NodeModel::new(
                    e.path,
                    e.title,
                );
                Some(result)
            }
            None => {
                None
            }
        }
    }
}