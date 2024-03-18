use actix_web::{web, Responder, HttpResponse};
use chrono::Utc;
use entity::rooms;
use sea_orm::{ActiveModelTrait, IntoActiveValue};
use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use uuid::Uuid;
use crate::AppState;

#[derive(Deserialize)]
pub struct Room {
    name: String,
}

pub async fn create_room(app_state: web::Data<AppState>, request_data: web::Json<Room>) -> impl Responder {
    let database_connection = &app_state.database_connection;

    let room = rooms::ActiveModel{
        name: Set(request_data.name.to_owned()),
        created_at: Set(Utc::now().naive_utc()),
        id: Uuid::new_v4().into_active_value(),
        ..Default::default()
    };

    room.insert(database_connection).await.expect("Could not insert room");

    HttpResponse::Ok().body("Created new room")
}