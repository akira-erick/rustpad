use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Content {
    body: String,
}

async fn get_content() -> impl Responder {
    let content = Content {
        body: "Hello, world!".to_string(),
    };
    HttpResponse::Ok().json(content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/rust/", web::get().to(get_content))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}