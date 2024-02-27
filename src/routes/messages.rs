// @todo We need to serve this over controllers over time.
use std::env;
use actix_web::{HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection};
use sea_orm::ActiveValue::Set;
use entity::messages;

pub async fn create_message() -> impl Responder {
    let database_url = env::var("DATABASE_URL").expect("The database url is not set");

    let db: DatabaseConnection = Database::connect(database_url).await.expect("We could not setup database connection");

    let message = messages::ActiveModel{
        body: Set("Hallo dit is mijn eerste bericht".to_owned()),
        date_time: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    message.insert(&db).await.expect("Could not insert message");

    HttpResponse::Ok().body("We just created something")
}
