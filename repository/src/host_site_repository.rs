use entity::host_site::Entity as host_site_entity;
use entity::host_site::Model as host_site_model;
use sea_orm::{DatabaseConnection, DbErr};
use sea_orm::{entity::*, query::*};
use entity::host_site;

pub async fn find_by_host_id(id: i32, connection: &DatabaseConnection)
    -> Result<Option<host_site_model>, DbErr>
{
    let result = host_site_entity::find()
        .filter(host_site::Column::HostId.eq(id))
        .one(connection)
        .await;
    result
}