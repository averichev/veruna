use async_trait::async_trait;
use sea_orm::{Database, DbConn, DbErr};
use entity::nodes::{Entity, Model};
use sea_orm::{entity::*, query::*};
use entity::prelude::Nodes;
use veruna_domain::nodes::{Node, NodeModel, NodesRepository};

pub(crate) struct NodesRepositoryImpl {
    connection: DbConn,
}

impl NodesRepositoryImpl {
    pub(crate) async fn new(database_url: &String) -> Box<dyn NodesRepository> {
        let connection = Database::connect(database_url)
            .await
            .expect("Failed to setup the database");
        let result = NodesRepositoryImpl { connection };
        Box::new(result)
    }
}

#[async_trait(? Send)]
impl NodesRepository for NodesRepositoryImpl {
    async fn find_path(&self, path: String) -> Option<Box<dyn Node>> {
        let entity = Entity::find()
            .filter(<Nodes as EntityTrait>::Column::Path.eq(path))
            .one(&self.connection)
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