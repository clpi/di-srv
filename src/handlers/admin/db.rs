use crate::{models::{Response, UserIn}, state::State, handlers::{user::*, auth::validate}};
use actix_session::Session;
use actix_web::{ Error,
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};

pub fn routes(base: &str) -> actix_web::Scope {
    scope(base)
        .route("/up", get().to(db_up))
        .route("/down", get().to(db_down))
        .service(table_routes("/{table}"))
}

pub fn table_routes(base: &str) -> actix_web::Scope {
    scope("/{table}")
        .route("", get().to(get_all_table))
        .route("/down", get().to(table_down))
        .route("/up", get().to(table_up))
}

pub async fn db_up(data: web::Data<State>) -> Result<HttpResponse, Error> {
    let db = data.db.lock().unwrap();
    match db.clone().init().await {
        Ok(_) => Ok(HttpResponse::Ok().body("Success")),
        Err(err) => Err(actix_web::error::ErrorUnauthorized(err))
    }
}

pub async fn db_down(data: web::Data<State>) -> HttpResponse {
    match &data.db.lock().unwrap().clone().down().await {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(_) => HttpResponse::InternalServerError().body("Could not take down DB")
    }
}

pub async fn db_reset(data: web::Data<State>) -> HttpResponse {
    match &data.db.lock().unwrap().clone().down().await {
        Ok(db) => {
            match db.clone().init().await {
                Ok(_db) => HttpResponse::Ok().body(""),
                Err(_) => HttpResponse::InternalServerError().body(""),
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Could not take down DB")
    }
}

pub async fn get_all_table(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn table_up(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn table_down(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("")
}
