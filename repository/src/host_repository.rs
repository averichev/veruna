use std::fmt::Error;
use entity::host::Entity as HostEntity;
use entity::host::Model as HostModel;
use sea_orm::{DatabaseConnection, DbErr};
use sea_orm::{entity::*, query::*};
use entity::host;

pub async fn find_by_name(name: &str, connection: &DatabaseConnection) -> Result<Option<HostModel>, DbErr>{
    let host = HostEntity::find()
        .filter(host::Column::Name.eq(name))
        .one(connection)
        .await;
    host
}