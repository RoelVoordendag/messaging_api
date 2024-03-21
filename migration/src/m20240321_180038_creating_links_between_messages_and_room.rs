use sea_orm_migration::prelude::*;
use crate::m20240303_211323_creation_users_rooms::Rooms;
use crate::m20220101_000001_create_table::Messages;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MessageRoom::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MessageRoom::MessageId).uuid().not_null())
                    .col(ColumnDef::new(MessageRoom::RoomId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .table(MessageRoom::Table)
                            .col(MessageRoom::MessageId)
                            .col(MessageRoom::RoomId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-messages_room-message-id")
                            .from(MessageRoom::Table, MessageRoom::MessageId)
                            .to(Messages::Table, Messages::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-messages_room-room-id")
                            .from(MessageRoom::Table, MessageRoom::RoomId)
                            .to(Rooms::Table, Rooms::Id)
                    )
                    .to_owned(),
            ).await

        // manager.create_foreign_key(
        //     ForeignKey::create()
        //         .name("fk-messages_room-message-id")
        //         .from(MessageRoom::Table, MessageRoom::MessageId)
        //         .to(Messages::Table, Messages::Id)
        //         .to_owned()
        // ).await.expect("Creating foreign key fk-messages_room-message_id did not succeed");
        //
        // manager.create_foreign_key(
        //     ForeignKey::create()
        //         .name("fk-messages_room-room-id")
        //         .from(MessageRoom::Table, MessageRoom::RoomId)
        //         .to(Rooms::Table, Rooms::Id)
        //         .to_owned()
        // ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MessageRoom::Table).to_owned())
            .await.expect("Something went wrong with dropping the table");

        manager.drop_foreign_key(
            ForeignKey::drop()
                .name("fk-messages_room-message-id")
                .to_owned()
        ).await.expect("Something went wrong removing index fk-messages_room-message-id");

        manager.drop_foreign_key(
            ForeignKey::drop()
                .name("fk-messages_room-room-id")
                .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum MessageRoom {
    Table,
    RoomId,
    MessageId,
}
