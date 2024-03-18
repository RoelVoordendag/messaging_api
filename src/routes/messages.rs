use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, IntoActiveValue};
use sea_orm::ActiveValue::Set;
use entity::messages;
use serde::Deserialize;
use uuid::Uuid;
use crate::AppState;

#[derive(Deserialize)]
pub struct Message {
    body: String,
    user_id: String,
}

pub async fn create_message(app_state: web::Data<AppState>, request_data: web::Json<Message>) -> impl Responder {
    let database_connection = &app_state.database_connection;

    let user_id = Uuid::parse_str(&request_data.user_id).unwrap();

    let message = messages::ActiveModel{
        body: Set(request_data.body.to_owned()),
        date_time: Set(Utc::now().naive_utc()),
        id: Uuid::new_v4().into_active_value(),
        user_id: user_id.into_active_value(),
    };

    message.insert(database_connection).await.expect("Could not insert message");

    HttpResponse::Ok().body("Created new Message")
}
