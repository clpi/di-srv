use actix_session::Session;
use crate::{state::State, models::UserIn};
use actix_web::{ Error, cookie::Cookie,
    get, post, put,
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse, Scope,
};

pub fn routes(base: &str) -> actix_web::Scope {
    scope(base)
        .route("", get().to(refresh_session))
        .route("/check", get().to(check_session))
}

pub async fn check_session(
    session: Session,
    req: HttpRequest) -> actix_web::Result<HttpResponse>
{
    let sess: Result<Option<UserIn>, Error> = session.get("uid");
    match sess {
        Ok(Some(user)) => {
            Ok(HttpResponse::Ok()
                .json(user))
        }
        _ => Ok(HttpResponse::NotFound()
                .json(false))
    }
}

pub(crate) fn validate(session: &Session) -> Result<UserIn, actix_web::HttpResponse> {
    let user: Option<UserIn> = session.get("uid").unwrap_or(None);
    match user {
        Some(user) => { session.renew(); Ok(user) },
        None => Err(HttpResponse::Unauthorized().json("Unauthorized"))
    }
}

pub async fn logout_session(id: Session, session: Session) -> Result<HttpResponse, HttpResponse> {
    let sess: Result<Option<UserIn>, Error> = session.get("uid");
    match sess {
        Ok(Some(_user)) => {
            session.purge();
            Ok(HttpResponse::Ok()
                .set_header("authorization", "false")
                .del_cookie(&Cookie::named("r-auth-cookie"))
                .del_cookie(&Cookie::named("auth-session"))
                .body("User logged out"))
        }
        _ => Err(HttpResponse::NotFound().body("No user to log out")),
    }
}

pub async fn refresh_session(
    (id,  _data): (Session, web::Data<State>)) -> HttpResponse
{
    match id.get::<usize>("id") {
        Ok(id) => {
            HttpResponse::Ok()
                .set_header("authorization", "true")
                .json(true)
        },
        Err(_) => HttpResponse::Gone()
            .set_header("authorization", "false")
            .json(false)

    }
}
