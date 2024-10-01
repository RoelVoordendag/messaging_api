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
                    .table(RoomsUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RoomsUsers::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(RoomsUsers::UserId).uuid().not_null())
                    .col(ColumnDef::new(RoomsUsers::RoomId).uuid().not_null())
                    .to_owned(),
            )
            .await
            .expect("Something went wrong with creating rooms users");

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-rooms-users-users")
                    .from(RoomsUsers::Table, RoomsUsers::UserId)
                    .to(Users::Table, Users::Id)
                    .to_owned(),
            )
            .await
            .expect("Something went wrong with setting foreign key rooms users user");

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-rooms-users-rooms")
                    .from(RoomsUsers::Table, RoomsUsers::RoomId)
                    .to(Rooms::Table, Rooms::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(ForeignKey::drop().name("fk-rooms-users-users").to_owned())
            .await
            .expect("Could not remove constaint fk-rooms-users-users");

        manager
            .drop_foreign_key(ForeignKey::drop().name("fk-rooms-users-rooms").to_owned())
            .await
            .expect("Could not remove contraint fk-rooms-users-rooms");

        manager
            .drop_table(Table::drop().table(RoomsUsers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RoomsUsers {
    Table,
    Id,
    UserId,
    RoomId,
}
