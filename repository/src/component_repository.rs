use sea_orm::{DatabaseConnection, DbErr, entity::*, query::*};
use entity::component::Entity as component_entity;
use entity::component::Model as component_model;
use entity::component;

pub async fn find_components_by_id(connection: &DatabaseConnection, nodes_id: Vec<i32>)
                                  -> Result<Vec<component_model>, DbErr> {
    let result = component_entity::find()
        .filter(component::Column::Id.is_in(nodes_id))
        .all(connection)
        .await;
    result
}