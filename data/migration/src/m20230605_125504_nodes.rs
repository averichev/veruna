use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Nodes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Nodes::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment()
                    )
                    .col(
                        ColumnDef::new(Nodes::Title)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Nodes::Lft)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Nodes::Rgt)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Nodes::Level)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Nodes::Path)
                            .string()
                            .not_null(),
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop().table(Nodes::Table).to_owned()
            )
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Nodes {
    Table,
    Id,
    Title,
    Lft,
    Rgt,
    Level,
    Path,
}
