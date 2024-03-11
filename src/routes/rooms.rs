use std::env;
use actix_web::{web, Responder, HttpResponse};
use chrono::Utc;
use entity::rooms;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection};
use sea_orm::ActiveValue::Set;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Room {
    name: String,
}

pub async fn create_room(request_data: web::Json<Room>) -> impl Responder {
    let db_url = env::var("DATABASE_URL").expect("Database url is not set in env");

    let db_connection: DatabaseConnection = Database::connect(db_url).await.expect("We could not setup a db connection");

    let room_entity = rooms::ActiveModel{
        name: Set(request_data.name.to_owned()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    room_entity.insert(&db_connection).await.expect("Could not insert room");

    HttpResponse::Ok().body("Created new room")
}