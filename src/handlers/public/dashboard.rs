use std::collections::HashMap;
use crate::state::State;
use actix_web::{ get,
    Scope, web::{scope, self},
    HttpResponse,
};

pub fn routes(base: &str) -> Scope {
    scope(base)
        .service(dashboard)
}

#[get("/")]
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
