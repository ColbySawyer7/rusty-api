use actix_web::{
    web, App, HttpResponse, HttpServer, Responder,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod schemas;
use schemas::user::User;
mod api;

#[actix_web::get("/greet")]
async fn greet() -> impl Responder {
    format!("Welcome my CRUD Rust API")
}

#[actix_web::get("/live")]
async fn live() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::get("/ready")]
async fn ready() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting server on port {port}");

    let user_db: api::user::UserDb = Arc::new(Mutex::new(HashMap::<u32, User>::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new()
            .app_data(app_data)
            .service(greet)
            .service(live)
            .service(ready)
            .service(api::user::create_user)
            .service(api::user::get_user)
            .service(api::chat::ask_chat)
    })
    .bind(("127.0.0.1", port))?
    .workers(2)
    .run()
    .await
}
