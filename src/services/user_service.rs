use std::{io::ErrorKind, str::FromStr};

use entity::users;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

pub struct UserService {
    pub database_connection: DatabaseConnection,
}

impl UserService {
    pub async fn user_exist(&self, user_id: String) -> bool {
        println!("test");

        let uuid = match Uuid::try_parse(&user_id) {
            Ok(uuid) => uuid,
            Err(_) => return false,
        };

        let user = users::Entity::find_by_id(uuid)
            .one(&self.database_connection)
            .await
            .expect("123");

        println!("{:?}", uuid);
        println!("{:?}", user);

        return true;

        // return !user.is_none();
    }
}
