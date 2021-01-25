use actix_web::{ get,
    Scope, web::scope,
    web::{self, ServiceConfig},
    HttpResponse,
};
use std::collections::HashMap;
use crate::state::State;

pub fn routes(base: &str) -> Scope {
    scope(base)
        .service(dashboard)
}

#[get("/dashboard")]
pub async fn dashboard(
    _id: actix_session::Session,
    _req: actix_web::HttpRequest,
    _query: web::Query<HashMap<String, String>>,
    data: web::Data<State>,) -> HttpResponse
{
    let _db = data.db.lock().unwrap();
    let mut ctx = tera::Context::new();
    let s = data.tera.render("dashboard.html", &ctx)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))
            .unwrap_or_default();
    HttpResponse::Ok().content_type("text/html").body(s)
}
