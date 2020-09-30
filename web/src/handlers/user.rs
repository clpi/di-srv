use crate::context::Context;
use actix_web::{get, web, HttpResponse};
use divdb::models::user::User;

pub fn routes(cfg: &mut web::ServiceConfig) {} 

pub async fn get_all(data: web::Data<Context>) -> HttpResponse {
    let users: Vec<User> = User::get_all(&data.db).await.unwrap();
    HttpResponse::Ok().body(serde_json::to_string(&users).unwrap())
}

pub async fn get_by_id(data: web::Data<Context>, id: web::Path<i32>) -> HttpResponse {
    let user: User = User::get_by_id(&data.db, *id).await.unwrap();
    println!("User: {}", serde_json::to_string(&user).unwrap());
    HttpResponse::Ok().body(serde_json::to_string(&user).unwrap())
}

pub async fn get_by_username() {}

pub async fn add_user() {}

pub async fn update_user() {}

