mod routes;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use entity::messages;
use uuid::Uuid;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection};
use sea_orm::ActiveValue::Set;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    let db: DatabaseConnection = Database::connect("postgres://messaging_api:messaging_api@messaging_api_db.docker/messaging_api").await.expect("Something went wrong");
    
    HttpResponse::Ok().body("Hey there!");

    let messaage = messages::ActiveModel{
        body: Set("Hallo dit is mijn eerste bericht".to_owned()),
        date_time: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    messaage.insert(&db).await.expect("TODO: panic message");

    HttpResponse::Ok().body("We just created something")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: DatabaseConnection = Database::connect("postgres://messaging_api:messaging_api@messaging_api_db.docker/messaging_api").await.expect("Something went wrong");

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/app")
                    .route("/hello", web::get().to(routes::routes::hello))
            )
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}