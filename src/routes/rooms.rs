use crate::services::user_service::UserService;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use entity::{rooms, users};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, IntoActiveValue};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateRoomRequest {
    name: String,
    user_id: String,
    chat_user_id: String,
}

pub async fn create_room(
    app_state: web::Data<AppState>,
    request_data: web::Json<CreateRoomRequest>,
) -> impl Responder {
    let database_connection = &app_state.database_connection;

    let user_service = UserService {
        database_connection: app_state.database_connection.to_owned(),
    };

    if user_service
        .user_exist(request_data.user_id.to_owned())
        .await
        == false
        || user_service
            .user_exist(request_data.chat_user_id.to_owned())
            .await
            == false
    {
        return HttpResponse::NotAcceptable().body("Users do not exist");
    }

    // @todo handle the creation correctly
    //  Need to fill the connection table

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
