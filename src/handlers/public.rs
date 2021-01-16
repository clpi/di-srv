use uuid::Uuid;
use std::collections::HashMap;
use actix_session::Session;
use crate::{state::State, models::request::AuthRequest};
use actix_web::{FromRequest, Scope,
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpResponse, HttpRequest
};
use div_db::models::{Record, User, UserInfo,};
use serde::{Serialize, Deserialize};

pub fn public_routes() -> Scope {
    scope("")
    // -------------- /user ------------------------- ///
        .service(resource("").route(get().to(get_all)))
        .service(scope("/{uid}")
        // -------------- /user/{uid} --------------------///
            .service(resource("")
                .route(get().to(get_by_id))
                .route(delete().to(delete_by_id))
            )
            // ------------ /user/{uid}/info/ -------- ///
            .service(resource("/info")
                .route(get().to(get_user_info))
                .route(put().to(update_user_info))
            )
        )
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
    data: web::Data<State>,) -> impl actix_web::Responder
{
    let db = &data.db.lock().unwrap();
    let s = if let Some(name) = query.get("name") {
        let mut ctx = tera::Context::new();
        ctx.insert("name", &name.to_owned());
        ctx.insert("text", &"Welcome!".to_owned());
        tmpl.render("search.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    } else {

        let headers: HashMap<String, String> = std::collections::HashMap::new();
        tmpl.render("index.html", &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
