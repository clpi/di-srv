pub mod user;
pub mod auth;
pub mod record;

use actix_web::{
    HttpServer, App, web, HttpRequest, HttpResponse, Responder, dev,
};
use actix_identity::{Identity, IdentityService, CookieIdentityPolicy};

pub async fn index(id: Identity) -> impl Responder {
    let res = match id.identity() {
        Some(id) => format!("Hello, {}", id),
        None => "Welcome newcomer!".to_string()
    };
    HttpResponse::Ok().body(res)
}
