mod routes;
mod services;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::env;

#[derive(Debug, Clone)]
struct AppState {
    database_connection: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("The database url is not set");
    let db_connection = Database::connect(database_url).await.unwrap();

    let app_state = AppState {
        database_connection: db_connection,
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(
                web::scope("/api")
                    .app_data(web::JsonConfig::default())
                    .route(
                        "/messages",
                        web::post().to(routes::messages::create_message),
                    )
                    .route("/users", web::post().to(routes::users::create_user))
                    .route("/users", web::get().to(routes::users::search_user))
                    .route(
                        "/users/create",
                        web::post().to(routes::users::get_or_create_user),
                    )
                    .route("/rooms", web::post().to(routes::rooms::create_room)),
            )
    })
    .bind(("127.0.0.1", 4040))?
    .run()
    .await
}
