use crate::models::Response;
use actix_web::{
    HttpResponse,
    web::{self, scope, resource, get, post},
};
use serde::{Deserialize, Serialize};

pub fn routes(base: &str) -> actix_web::Scope {
    scope(base)
        .route("", get().to(server_info))
        .route("/up", post().to(server_up))
        .route("/down", post().to(server_down))
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
