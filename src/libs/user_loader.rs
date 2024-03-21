use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;
use entity::prelude::Users;

pub struct UserLoader {
    pub db: DatabaseConnection
}

impl UserLoader {
    pub async fn user_exist(&self, user_id: Uuid) -> bool {
        let db = self.db.clone();

        return match Users::find_by_id(user_id).one(&db).await {
            Ok(result) => result.is_some(),
            Err(_) => false
        };
    }
}