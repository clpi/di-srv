pub mod db;
pub mod server;
pub mod user;

use crate::{models::{Response, UserIn}, state::State, handlers::{user::*, auth::validate}};
use actix_session::Session;
use actix_web::{ Error,
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};


// pub struct AdminRouteGuard;

// impl Guard for AdminRouteGuard {
//     fn check(&self, request: &RequestHead, session: Session) -> bool {
//         session.get::<usize>("uid").unwrap() == uuid::Uuid::new_v4()
//     }
// }

pub fn routes(base: &str) -> actix_web::Scope {
    scope(base)
        .route("", get().to(check_auth))
        .service(self::db::routes("/db"))
        .service(self::server::routes("/server"))
        .service(self::user::routes("/user"))
}

pub async fn check_auth(session: Session, data: web::Data<State>) -> Result<HttpResponse, Error> {
    let user = validate(&session)?;
    Ok(HttpResponse::Ok().json(user))
}


