use actix_web::client::Client;
use div_cloud::cognito::types::*;
use actix_session::Session;
use serde::{Serialize, Deserialize};
use crate::{state::State, models::UserIn};
use actix_web::{ Error, cookie::Cookie,
    get, post, put,
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse, Scope,
};
use div_db::models::user::*;

pub fn routes(base: &str) -> Scope {
    scope(base)
        .route("", get().to(get_jwt))
        .route("", post().to(refresh_jwt))
}

pub async fn get_jwt(
    req: HttpRequest,
    data: web::Data<State>,
) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .body("GET /api/auth/jwt"))
}

pub async fn refresh_jwt(
    req: HttpRequest,
    data: web::Data<State>,
) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .body("GET /api/auth/jwt/refresh"))
}
