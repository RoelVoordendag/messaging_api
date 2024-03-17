use serde::Deserialize;
use entity::users;
use entity::users::Entity as UserLoader;
use actix_web::{web, Responder, HttpResponse};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, QueryFilter, ColumnTrait, EntityTrait};
use sea_orm::ActiveValue::Set;
use crate::AppState;

#[derive(Deserialize)]
pub struct User {
    name: String,
}

pub async fn create_user(app_state: web::Data<AppState> , request_data: web::Json<User>) -> impl Responder {
    let database_connection = &app_state.database_connection;

    let user: Option<users::Model> = UserLoader::find()
        .filter(users::Column::Name.contains(request_data.name.to_owned()))
        .one(database_connection)
        .await.expect("Something went wrong collecting data");

    if user != None {
        return HttpResponse::NotAcceptable().body("User already exists");
    }

    let user_entity = users::ActiveModel{
        name: Set(request_data.name.to_owned()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    user_entity.insert(database_connection).await.expect("Something went wrong with creation user");

    return HttpResponse::Ok().body("Created new user")
}