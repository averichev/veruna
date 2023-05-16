use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter().table(Site::Table)
                    .add_column(
                        ColumnDef::new(Site::Description).text()
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter().table(Site::Table)
                    .drop_column(Site::Description)
                    .to_owned()
            )
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
    Description,
}




