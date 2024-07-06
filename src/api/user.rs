use actix_web::{
    error::Error, error::ErrorNotFound, web, HttpResponse, Responder,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::schemas::user::{User, CreateUserResponse};

pub type UserDb = Arc<Mutex<HashMap<u32, User>>>;

#[actix_web::get("/users/{id}")]
pub async fn get_user(user_id: web::Path<u32>, db: web::Data<UserDb>) -> Result<impl Responder, Error> {
    let user_id = user_id.into_inner();
    let db = db.lock().unwrap();

    match db.get(&user_id) {
        Some(user_data) => Ok(HttpResponse::Ok().json(user_data)),
        None => Err(ErrorNotFound("User not found")),
    }
}

#[actix_web::post("/users")]
pub async fn create_user(user_data: web::Json<User>, db: web::Data<UserDb>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let new_id = db.keys().max().unwrap_or(&0) + 1;
    let name = user_data.name.clone();
    db.insert(new_id, user_data.into_inner());
    HttpResponse::Created().json(CreateUserResponse { id: new_id, name })
}
