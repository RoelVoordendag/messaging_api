use std::env;
use actix_web::{web, Responder, HttpResponse};
use chrono::Utc;
use entity::rooms;
use sea_orm::{ActiveModelTrait};
use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use crate::AppState;

#[derive(Deserialize)]
pub struct Room {
    name: String,
}

pub async fn create_room(app_state: web::Data<AppState>, request_data: web::Json<Room>) -> impl Responder {
    let database_connection = &app_state.database_connection;

    let room_entity = rooms::ActiveModel{
        name: Set(request_data.name.to_owned()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    room_entity.insert(database_connection).await.expect("Could not insert room");

    HttpResponse::Ok().body("Created new room")
}