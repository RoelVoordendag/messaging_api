mod routes;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .app_data(web::JsonConfig::default())
                    .route("/message", web::post().to(routes::messages::create_message))
            )
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}