use std::env;
use serde::Deserialize;
use entity::users;
use entity::users::Entity as UserLoader;
use actix_web::{web, Responder, HttpResponse};
use chrono::Utc;
use sea_orm::{Database, DatabaseConnection, ActiveModelTrait, QueryFilter, ColumnTrait, EntityTrait};
use sea_orm::ActiveValue::Set;

#[derive(Deserialize)]
pub struct User {
    name: String,
}

pub async fn create_user(request_data: web::Json<User>) -> impl Responder {
    let database_url = env::var("DATABASE_URL").expect("The database url is not set");

    let db: DatabaseConnection = Database::connect(database_url).await.expect("We could not setup database connection");

    let user: Option<users::Model> = UserLoader::find()
        .filter(users::Column::Name.contains(request_data.name.to_owned()))
        .one(&db)
        .await.expect("Something went wrong collecting data");

    if user != None {
        // @todo we no return a error
        println!("test, {}", user.unwrap().name);
    }


    let user_entity = users::ActiveModel{
        name: Set(request_data.name.to_owned()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    user_entity.insert(&db).await.expect("Something went wrong with creation user");

    HttpResponse::Ok().body("Created new user")
}