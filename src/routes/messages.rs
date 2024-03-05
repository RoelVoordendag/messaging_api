use std::env;
use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection};
use sea_orm::ActiveValue::Set;
use entity::messages;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Message {
    body: String,
}

pub async fn create_message(message: web::Json<Message>) -> impl Responder {
    let database_url = env::var("DATABASE_URL").expect("The database url is not set");

    let db: DatabaseConnection = Database::connect(database_url).await.expect("We could not setup database connection");

    let message_entity = messages::ActiveModel{
        body: Set(message.body.to_owned()),
        date_time: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    message_entity.insert(&db).await.expect("Could not insert message");

    HttpResponse::Ok().body("Created new Message")
}
