use crate::context::Context;
use com::auth::*;
use divdb::models::user::{User, UserLogin};
use actix_web::{web::{self, resource, ServiceConfig, scope, put, get, delete, post}, HttpResponse, HttpRequest};
use actix_identity::{Identity, IdentityService, CookieIdentityPolicy};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
    .service(scope("/auth")
        .service(resource("/login")
            .route(post().to(login)))
        .service(resource("/signup")
            .route(post().to(signup)))
        .service(resource("/logout")
            .route(post().to(logout)))
        .service(resource("/refresh")
            .route(post().to(refresh_login)))
    );
} 

pub async fn signup(
    (req, user, data): (HttpRequest, web::Json<User>, web::Data<Context>)
) -> HttpResponse {
    HttpResponse::Ok().body("signup")
}

pub async fn login(
    (id, req, user, data): (Identity, HttpRequest, web::Json<UserLogin>, web::Data<Context>)
) -> HttpResponse {
    HttpResponse::Ok().body("login")
}

pub async fn logout(
    (id, req, user, data): (Identity, HttpRequest, web::Json<UserLogin>, web::Data<Context>)
) -> HttpResponse {
    HttpResponse::Ok().body("logout")
}

pub async fn refresh_login(
    (id, req, user, data): (Identity, HttpRequest, web::Json<UserLogin>, web::Data<Context>)
) -> HttpResponse {
    HttpResponse::Ok().body("refresh_login")
}
