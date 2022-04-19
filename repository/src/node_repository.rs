use entity::node::Entity as node_entity;
use entity::node::Model as node_model;
use entity::node_site::Entity as node_site_entity;
use entity::node_site::Model as node_site_model;
use sea_orm::{DatabaseConnection, DbErr};
use sea_orm::{entity::*, query::*};
use entity::node;
use entity::node_site;

pub async fn find_node_by_code(code: &String, connection: &DatabaseConnection)
                               -> Result<Option<node_model>, DbErr>
{
    let result = node_entity::find()
        .filter(node::Column::Code.eq(code.as_str()))
        .one(connection)
        .await;
    result
}


pub async fn find_node_by_id(id: i32, connection: &DatabaseConnection)
                               -> Result<Option<node_model>, DbErr>
{
    let result = node_entity::find()
        .filter(node::Column::Id.eq(id))
        .one(connection)
        .await;
    result
}

pub async fn find_node_site_relation(node: node_model, connection: &DatabaseConnection)
                                     -> Result<Option<node_site_model>, DbErr>
{
    let result = node_site_entity::find()
        .filter(node_site::Column::NodeId.eq(node.id))
        .one(connection)
        .await;
    result
}

pub async fn find_path(node: Vec<String>, connection: &DatabaseConnection, root_id: i32)
                       -> Result<Vec<node_model>, DbErr>
{
    let mut any_condition: Condition = Condition::any();
    for (index, e) in node.into_iter().enumerate(){
        let level: i32 = index as i32;
        let all_condition = Condition::all()
            .add(node::Column::Code.eq(e))
            .add(node::Column::Level.eq(level))
            .add(node::Column::Root.eq(root_id));
        any_condition = any_condition.add(all_condition);
    }

    let result = node_entity::find()
        .filter(any_condition)
        .all(connection)
        .await;

    result
}