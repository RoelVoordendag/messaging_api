use sea_orm_migration::prelude::*;
use crate::m20240303_211323_creation_users_rooms::{Rooms, Users};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRoom::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserRoom::RoomId).uuid().not_null())
                    .col(ColumnDef::new(UserRoom::UserId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .table(UserRoom::Table)
                            .col(UserRoom::RoomId)
                            .col(UserRoom::UserId)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_room-user-id")
                            .from(UserRoom::Table, UserRoom::UserId)
                            .to(Users::Table, Users::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_room-room-id")
                            .from(UserRoom::Table, UserRoom::RoomId)
                            .to(Rooms::Table, Rooms::Id)
                    )
                    .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRoom::Table).to_owned())
            .await.expect("Something went wrong with dropping the table");

        manager.drop_foreign_key(
            ForeignKey::drop()
                .name("fk-user_room-user-id")
                .to_owned()
        ).await.expect("Something went wrong removing index fk-user_room-user-id");

        manager.drop_foreign_key(
            ForeignKey::drop()
                .name("fk-user_room-room-id")
                .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum UserRoom {
    Table,
    RoomId,
    UserId,
}
