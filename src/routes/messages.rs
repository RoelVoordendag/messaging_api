use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveValue, LoaderTrait};
use sea_orm::ActiveValue::Set;
use entity::{messages, rooms};
use serde::Deserialize;
use uuid::Uuid;
use entity::prelude::{MessageRoom, Messages, Rooms};
use libs::uuid_util::UuidService;
use crate::{AppState, libs};

#[derive(Deserialize)]
pub struct Message {
    body: String,
    user_id: String,
}

pub async fn create_message(app_state: web::Data<AppState>, request_data: web::Json<Message>) -> impl Responder {
    let database_connection = &app_state.database_connection;

    if UuidService::is_uuid_valid(&request_data.user_id) == false {
        return HttpResponse::NotAcceptable().body("User id is not a valid UUID");
    }

    let user_id = Uuid::parse_str(&request_data.user_id).unwrap();

    // @todo we need to check if the user really exist otherwise give error

    let message = messages::ActiveModel{
        body: Set(request_data.body.to_owned()),
        date_time: Set(Utc::now().naive_utc()),
        id: Uuid::new_v4().into_active_value(),
        user_id: user_id.into_active_value(),
    };

    match message.insert(database_connection).await {
        Ok(_) => HttpResponse::Ok().body("Created new Message."),
        Err(_) => HttpResponse::InternalServerError().body("Could not insert message")
    }
}

pub async fn get_messages(app_state: web::Data<AppState>) -> impl Responder {
    let database_connection = &app_state.database_connection;

    let found: Vec<entity::messages::Model> = Messages::find().all(database_connection).await.expect("testing");
    let rooms: Vec<Vec<rooms::Model>> = found.load_many_to_many(Rooms, MessageRoom, database_connection).await.expect("lmao");

    for message in found.iter()  {
        println!("{message:?}\n");
    }

    for room in rooms.iter()  {
        println!("{room:?}\n");
    }

    HttpResponse::Ok().body("these are the messages")
}

