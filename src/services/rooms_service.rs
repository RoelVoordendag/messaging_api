use entity::rooms_users;
use sea_orm::{DatabaseConnection, EntityTrait, Set, TransactionTrait};
use uuid::Uuid;

pub struct RoomsService {
    pub data_baseconnection: DatabaseConnection,
}

impl RoomsService {
    // introduce rollback when something fail
    pub async fn connect_users_to_room(&self, room_id: Uuid, user_ids: Vec<String>) -> bool {
        let transaction = self.data_baseconnection.begin().await.unwrap();

        for user_id in user_ids.iter() {
            let validated_user_id = match Uuid::try_parse(user_id) {
                Ok(uuid) => uuid,
                Err(_) => return false,
            };

            rooms_users::Entity::insert(rooms_users::ActiveModel {
                user_id: Set(validated_user_id),
                room_id: Set(room_id),
            })
            .exec(&transaction)
            .await
            {
                transaction.rollback().await.unwrap(); // Rollback if insert fails
                return false;
            }.
        }

        transaction.commit().await.unwrap();

        return true;
    }
}
