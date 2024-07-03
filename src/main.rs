use actix_web::{
    error::Error, error::ErrorNotFound, web, App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
}
#[derive(Serialize)]
struct CreateUserResponse {
    id: u32,
    name: String,
}

type UserDb = Arc<Mutex<HashMap<u32, User>>>;

#[actix_web::get("/greet")]
async fn greet() -> impl Responder {
    format!("Welcome my CRUD Rust API")
}

#[actix_web::get("/users/{id}")]
async fn get_user(user_id: web::Path<u32>, db: web::Data<UserDb>) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();
    let db = db.lock().unwrap();

    match db.get(&user_id) {
        Some(user_data) => Ok(HttpResponse::Ok().json(user_data)),
        None => Err(ErrorNotFound("User not found")),
    }
}

#[actix_web::post("/users")]
async fn create_user(user_data: web::Json<User>, db: web::Data<UserDb>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let new_id = db.keys().max().unwrap_or(&0) + 1;
    let name = user_data.name.clone();
    db.insert(new_id, user_data.into_inner());
    HttpResponse::Created().json(CreateUserResponse { id: new_id, name })
}

#[derive(Serialize, Deserialize)]
struct Prompt {
    question: String,
}

#[derive(Serialize)]
struct PromptResponse {
    question: String,
    answer: String,
}

#[actix_web::post("/chat")]
async fn ask_chat(
    prompt: web::Json<Prompt>,
) -> Result<impl Responder, Error> {
    let question = &prompt.question;

    let ollama = Ollama::new("http://localhost".to_string(), 11434);
    let model = "llama3:latest".to_string();

    match ollama
        .generate(GenerationRequest::new(model, question.clone()))
        .await
    {
        Ok(res) => {
            let response = PromptResponse {
                question: question.clone(),
                answer: res.response,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(_) => Err(actix_web::error::ErrorNotFound("User not found")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting server on port {port}");

    let user_db: UserDb = Arc::new(Mutex::new(HashMap::<u32, User>::new()));

    HttpServer::new(move || {
        let app_data = web::Data::new(user_db.clone());
        App::new()
            .app_data(app_data)
            .service(greet)
            .service(create_user)
            .service(get_user)
            .service(ask_chat)
    })
    .bind(("127.0.0.1", port))?
    .workers(2)
    .run()
    .await
}
