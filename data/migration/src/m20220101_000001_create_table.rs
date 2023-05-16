use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Site::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Site::Id)
                            .small_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Site::Name)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Site::Domain)
                            .string()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Site::Port)
                            .integer()
                            .null()
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Site::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Site {
    Table,
    Id,
    Name,
    Domain,
    Port,
}