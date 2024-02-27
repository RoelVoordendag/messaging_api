mod routes;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use entity::messages;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection};
use sea_orm::ActiveValue::Set;
use std::env;
use std::fmt::format;
use dotenv::dotenv;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn insert_message() -> impl Responder {
    let database_url = env::var("DATABASE_URL").expect("The database url is not set");

    let db: DatabaseConnection = Database::connect(database_url).await.expect("We could not setup database connection");
    
    HttpResponse::Ok().body("Hey there!");

    let message = messages::ActiveModel{
        body: Set("Hallo dit is mijn eerste bericht".to_owned()),
        date_time: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    message.insert(&db).await.expect("Could not insert message");

    HttpResponse::Ok().body("We just created something")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/app")
                    .route("/hello", web::get().to(routes::routes::hello))
            )
            .service(echo)
            .route("/hey", web::get().to(insert_message))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}