use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;
use entity::prelude::Rooms;

pub struct RoomLoader {
    pub db: DatabaseConnection,
}

impl RoomLoader {
    pub async fn room_exist(&self, room_id: Uuid) -> bool {

        return match Rooms::find_by_id(room_id).one(&self.db).await {
            Ok(result) => { return result.is_some() },
            Err(..) => false,
        }
    }
}