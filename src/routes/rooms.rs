use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use entity::rooms;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, IntoActiveValue};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RoomRequest {
    name: String,
}

pub async fn create_room(
    app_state: web::Data<AppState>,
    request_data: web::Json<RoomRequest>,
) -> impl Responder {
    let database_connection = &app_state.database_connection;

    // /**

    //     We need to recieve the current user and the one they are trying to connect with
    //     So
    //     created_by: user_id
    //     name: "De coole kamer"
    //     users: [
    //         { usersId }
    //     ]
    // */
    //
    let room = rooms::ActiveModel {
        name: Set(request_data.name.to_owned()),
        created_at: Set(Utc::now().naive_utc()),
        id: Uuid::new_v4().into_active_value(),
        ..Default::default()
    };

    room.insert(database_connection)
        .await
        .expect("Could not insert room");

    HttpResponse::Ok().body("Created new room")
}
