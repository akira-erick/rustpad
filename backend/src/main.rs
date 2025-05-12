use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use actix_web::web::Data;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Content {
    body: String,
}

struct AppState {
    store: Mutex<HashMap<String, Content>>,
}

async fn get_content(path: web::Path<String>, data: Data<AppState>) -> impl Responder {
    let path = path.into_inner();
    let store = data.store.lock().unwrap();
    if let Some(content) = store.get(&path) {
        HttpResponse::Ok().json(content)
    } else {
        HttpResponse::Ok().json(Content {
            body: "".to_string(),
        })
    }
}

async fn set_content(path: web::Path<String>, content: web::Json<Content>, data: Data<AppState>) -> impl Responder {
    let path = path.into_inner();
    let mut store = data.store.lock().unwrap();
    store.insert(path.clone(), content.into_inner());
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let app_state = web::Data::new(AppState {
        store: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/rust/{path}", web::get().to(get_content))
            .route("/rust/{path}", web::post().to(set_content))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}