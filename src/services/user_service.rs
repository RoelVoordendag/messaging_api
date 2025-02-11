use entity::users;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

pub struct UserService {
    pub database_connection: DatabaseConnection,
}

impl UserService {
    pub async fn user_exist(&self, user_id: String) -> bool {
        let uuid = match Uuid::try_parse(&user_id) {
            Ok(uuid) => uuid,
            Err(_) => return false,
        };

        let user = users::Entity::find_by_id(uuid)
            .one(&self.database_connection)
            .await
            .expect("Database connection is failing.");

        return !user.is_none();
    }
}
