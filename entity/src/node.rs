//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "node")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub code: String,
    pub lft: i32,
    pub rgt: i32,
    pub root: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::Root",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef,
}

impl ActiveModelBehavior for ActiveModel {}