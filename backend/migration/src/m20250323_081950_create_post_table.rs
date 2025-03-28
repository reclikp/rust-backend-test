use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Post::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Post::Title).string().not_null().unique_key())
                    .col(ColumnDef::new(Post::Content).string().null())
                    .col(ColumnDef::new(Post::Published).boolean().not_null().default(false))
                    .col(ColumnDef::new(Post::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Content,
    Published,
    CreatedAt,
}
