use uuid::Uuid;
use std::collections::HashMap;
use actix_session::Session;
use crate::{state::State, models::request::AuthRequest};
use actix_web::{FromRequest, Scope,
    Responder,
    get, post, put, delete,
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpResponse, HttpRequest
};
use div_db::models::{Record, User, UserInfo,};
use serde::{Serialize, Deserialize};

pub fn public_routes() -> Scope {
    scope("")
        .service(resource("").route(get().to(index)))
        .service(resource("dashboard").route(get().to(index)))
        .service(resource("users").route(get().to(users)))
        .service(resource("user").route(get().to(user)))
        .service(resource("login").route(get().to(login)))
}

pub(crate) async fn static_ind(_id: Session) -> impl Responder {
    //TODO Only works when run in root dir
    let html = String::from_utf8(std::fs::read("assets/static/templates/index.html").unwrap()).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

pub async fn index(
    id: actix_session::Session,
    req: actix_web::HttpRequest,
    query: web::Query<HashMap<String, String>>,
    data: web::Data<State>,) -> HttpResponse
{
    let db = &data.db.lock().unwrap();
    let s = if let Some(name) = query.get("name") {
        let mut ctx = tera::Context::new();
        ctx.insert("name", &name.to_owned());
        ctx.insert("text", &"Welcome!".to_owned());
        data.tera.render("search.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))
            .unwrap_or_default()
    } else {
        let h = req.headers().into_iter()
            .fold(HashMap::new(), |mut hm, (h, v)| {
                hm.insert(h.to_string(), v.to_str().unwrap_or_default().to_string());
                hm
            });
        let mut context = tera::Context::new();
        context.insert("headers", &h.to_owned());
        data.tera.render("index.html", &context)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))
            .unwrap_or_default()
    };
    HttpResponse::Ok().content_type("text/html").body(s)
}

pub async fn dashboard(
    _id: actix_session::Session,
    _req: actix_web::HttpRequest,
    _query: web::Query<HashMap<String, String>>,
    data: web::Data<State>,) -> HttpResponse
{
    let _db = &data.db.lock().unwrap();
    let mut ctx = tera::Context::new();
    let s = data.tera.render("dashboard.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))
            .unwrap_or_default();
    HttpResponse::Ok().content_type("text/html").body(s)
}

pub async fn login(
    _id: actix_session::Session,
    _req: actix_web::HttpRequest,
    _query: web::Query<HashMap<String, String>>,
    data: web::Data<State>,) -> Result<HttpResponse, actix_web::Error>
{
    let _db = &data.db.lock().unwrap();
    let mut ctx = tera::Context::new();
    let s = data.tera.render("login.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn users(
    _id: actix_session::Session,
    _req: actix_web::HttpRequest,
    _query: web::Query<HashMap<String, String>>,
    data: web::Data<State>,) -> Result<HttpResponse, actix_web::Error>
{
    let db = data.db.lock().unwrap();
    let users = User::get_all(&db).await.unwrap_or_default();
    let mut ctx = tera::Context::new();
    ctx.insert("users", &users.to_owned());
    let s = data.tera.render("users.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn user(
    _id: actix_session::Session,
    _req: actix_web::HttpRequest,
    _query: web::Query<HashMap<String, String>>,
    username: web::Path<String>,
    data: web::Data<State>,) -> Result<HttpResponse, actix_web::Error>
{
    let db = data.db.lock().unwrap();
    let user = User::get_by_username(&db, username.into_inner()).await.unwrap_or_default();
    let mut ctx = tera::Context::new();
    if let Some(user) = user {
        ctx.insert("user", &user.to_owned());
        let s = data.tera.render("user.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    } else {
        ctx.insert("status_code", &"404".to_string());
        ctx.insert("error", &"No user found".to_string());
        let s = data.tera.render("error.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }
}
