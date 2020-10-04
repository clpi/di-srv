use crate::{models::Response, state::State,};
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{
    http::{Cookie, HeaderName, HeaderValue},
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/admin")
            .service(
                scope("/db")
                    .service(resource("/up").route(get().to(db_up)))
                    .service(resource("/down").route(get().to(db_down)))
                    .service(
                        scope("/{table}")
                            .service(resource("").route(get().to(get_all_table)))
                            .service(resource("/down").route(get().to(table_down)))
                            .service(resource("/up").route(get().to(table_up))),
                    ),
            )
            .service(
                scope("/server")
                    .service(resource("").route(get().to(server_info)))
                    .service(resource("/up").route(post().to(server_up)))
                    .service(resource("/down").route(post().to(server_down))),
            ),
    );
}

pub async fn db_up(data: web::Data<State>) -> HttpResponse {
    match &data.db.clone().init().await {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(_) => HttpResponse::InternalServerError().body("Could not take down DB")
    }
}

pub async fn db_down(data: web::Data<State>) -> HttpResponse {
    match &data.db.clone().down().await {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(_) => HttpResponse::InternalServerError().body("Could not take down DB")
    }
}

pub async fn db_reset(data: web::Data<State>) -> HttpResponse {
    match &data.db.clone().down().await {
        Ok(db) => { 
            match db.clone().init().await {
                Ok(_db) => HttpResponse::Ok().body(""),
                Err(_) => HttpResponse::InternalServerError().body(""),
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Could not take down DB")
    }
}

pub async fn server_info() -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn server_up() -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn server_down() -> HttpResponse {
    HttpResponse::Ok().body("")
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

pub async fn run_cmd(cmd: web::Json<Cmd>) -> HttpResponse {
    use std::process::Command;
    let proc = Command::new("sh")
        .arg(&cmd.cmd)
        .status()
        .expect("Failed to execute cmd");
    if proc.success() {
        HttpResponse::Ok().json(Response::ok())
    } else {
        HttpResponse::Ok().json(Response::fail())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Cmd {
    cmd: String,
}
