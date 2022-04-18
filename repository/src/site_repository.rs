use entity::site::Entity as site_entity;
use entity::site::Model as site_model;
use entity::node_site::Model as node_site_model;
use entity::node_site::Entity as node_site_entity;
use sea_orm::{DatabaseConnection, DbErr};
use sea_orm::{entity::*, query::*};
use entity::site;
use entity::node_site;

pub async fn find_site_by_id(id: i32, connection: &DatabaseConnection)
                             -> Result<Option<site_model>, DbErr>
{
    let result = site_entity::find()
        .filter(site::Column::Id.eq(id))
        .one(connection)
        .await;
    result
}


pub async fn find_node_site_relation(site: site_model, connection: &DatabaseConnection)
                                     -> Result<Option<node_site_model>, DbErr>
{
    let result = node_site_entity::find()
        .filter(node_site::Column::SiteId.eq(site.id))
        .one(connection)
        .await;
    result
}