use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).text().not_null().primary_key())
                    .col(
                        ColumnDef::new(User::Discriminator)
                            .text()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Name).text().not_null())
                    .col(ColumnDef::new(User::Email).text().not_null().unique_key())
                    .col(ColumnDef::new(User::WebPage).text())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "discriminator")]
    Discriminator,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "email")]
    Email,
    #[sea_orm(iden = "web_page")]
    WebPage,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}
