use std::fmt::Error;
use entity::site::Entity as site_entity;
use entity::site::Model as site_model;
use sea_orm::{DatabaseConnection, DbErr};
use sea_orm::{entity::*, query::*};
use entity::site;

pub async fn find_site_by_id(id: i32, connection: &DatabaseConnection)
                             -> Result<Option<site_model>, DbErr>
{
    let result = site_entity::find()
        .filter(site::Column::Id.eq(id))
        .one(connection)
        .await;
    result
}