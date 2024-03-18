use sea_orm_migration::prelude::*;
use crate::m20240303_211323_creation_users_rooms::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                    Table::alter()
                        .table(Messages::Table)
                        .add_column(
                            ColumnDef::new(Messages::UserId)
                                .uuid()
                                .not_null()
                        )
                        .to_owned()
            ).await.expect("Something went wrong with inserting new column");

        manager.create_foreign_key(
            ForeignKey::create()
                .name("fk-message-user-id")
                .from(Messages::Table, Messages::UserId)
                .to(Users::Table, Users::Id).to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Messages::Table)
                    .drop_column(Messages::UserId).to_owned()
            ).await.expect("We could not remove the UserId table");

        manager.drop_foreign_key(
            ForeignKey::drop()
                .name("fk-message-user-id")
                .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum Messages {
    Table,
    UserId,
}
