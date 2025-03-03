pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240303_211323_creation_users_rooms;
mod m20240318_194802_setting_up_link_between_messages_and_users;
mod m20241001_181625_create_rooms_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240303_211323_creation_users_rooms::Migration),
            Box::new(m20240318_194802_setting_up_link_between_messages_and_users::Migration),
            Box::new(m20241001_181625_create_rooms_user_table::Migration),
        ]
    }
}
