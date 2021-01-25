use std::collections::HashMap;
use crate::state::State;
use actix_web::{ get,
    Scope, web::{scope, self},
    HttpResponse,
};

pub fn routes(base: &str) -> Scope {
    scope(base)
        .service(login)
        .service(signup)
}

#[get("/login")]
pub async fn login(
    _id: actix_session::Session,
    _req: actix_web::HttpRequest,
    _query: web::Query<HashMap<String, String>>,
    data: web::Data<State>,) -> Result<HttpResponse, actix_web::Error>
{
    let _db = data.db.lock().unwrap();
    let mut ctx = tera::Context::new();
    let s = data.tera.render("login.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/signup")]
pub async fn signup(
    _id: actix_session::Session,
    _req: actix_web::HttpRequest,
    _query: web::Query<HashMap<String, String>>,
    data: web::Data<State>,) -> Result<HttpResponse, actix_web::Error>
{
    let _db = data.db.lock().unwrap();
    let mut ctx = tera::Context::new();
    let s = data.tera.render("login.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
