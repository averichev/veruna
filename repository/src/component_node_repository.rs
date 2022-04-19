use sea_orm::{DatabaseConnection, DbErr, entity::*, query::*};
use entity::component_node::Entity as component_node_entity;
use entity::component_node::Model as component_node_model;
use entity::component_node;

pub async fn find_node_components(connection: &DatabaseConnection, node_id: i32)
                                  -> Result<Vec<component_node_model>, DbErr> {
    let result = component_node_entity::find()
        .filter(component_node::Column::NodeId.eq(node_id))
        .all(connection)
        .await;
    result
}