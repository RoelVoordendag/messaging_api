use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sea_orm::{ActiveModelTrait};
use sea_orm::ActiveValue::Set;
use entity::messages;
use serde::Deserialize;
use crate::AppState;

#[derive(Deserialize)]
pub struct Message {
    body: String,
}

pub async fn create_message(app_state: web::Data<AppState>, message: web::Json<Message>) -> impl Responder {
    let database_connection = &app_state.database_connection;

    let message_entity = messages::ActiveModel{
        body: Set(message.body.to_owned()),
        date_time: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    message_entity.insert(database_connection).await.expect("Could not insert message");

    HttpResponse::Ok().body("Created new Message")
}
