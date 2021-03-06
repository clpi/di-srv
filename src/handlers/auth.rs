pub mod site;
pub mod cognito;
pub mod jwt;
pub mod session;

use actix_session::Session;
use serde::{Serialize, Deserialize};
use crate::{state::State, models::UserIn};
use actix_web::{ Error, cookie::Cookie,
    get, post, put,
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse, Scope,
};
use div_db::models::user::*;

#[derive(Serialize, Deserialize)]
pub struct CognitoIn {}

pub fn routes(base: &str) -> Scope {
    scope(base)
        .service(self::site::routes(""))
        .service(self::cognito::routes("/cg"))
        .service(self::jwt::routes("/jwt"))
        .service(self::session::routes("/sess"))
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

pub async fn register_user(
    user: web::Json<UserRegister>,
    data: web::Data<State>,
) -> actix_web::Result<HttpResponse> {
    let db = data.db.lock().unwrap();
    let user = user.into_inner();
    let hash = crate::auth::PwVerifier::new()
        .hash(user.password.as_str())
        .expect("Could not hash password");
    let user = User::new(user.email, user.username, Some(hash));
    match user.insert_db(&db).await {
        Ok(user) => Ok(HttpResponse::Created().json(&user)),
        Err(e) =>Ok(HttpResponse::NotModified()
            .body(format!("User already exists, or other error... {}", e)))
    }
}

pub async fn signin_user(
    user: web::Json<UserLogin>,
    data: web::Data<State>,
) -> actix_web::Result<HttpResponse> {
    let db = data.db.lock().unwrap();
    let user = user.into_inner();
    let ver = crate::auth::PwVerifier::new();
    match User::get_by_username(&db, user.username).await {
        Ok(Some(duser)) => match ver.verify(user.password.as_str(), &duser.clone().password.unwrap()) {
            Ok(_) => return Ok(HttpResponse::Accepted()
                .set_header("auth", "false")
                .json(&duser)),
            Err(e) => return Ok(HttpResponse::Unauthorized()
                .set_header("auth", "false")
                .body(format!("Username or password incorrect: {}", e)))
        },
        Ok(None) => return Ok(HttpResponse::NotFound()
            .set_header("auth", "false")
            .body("No user found")),
        Err(e) => return Ok(HttpResponse::InternalServerError()
            .body(format!("could not reach server: {}", e)))
    }
}

pub async fn check_session_with_user(
    (session, req, user, data): (
        Session,
        HttpRequest,
        web::Json<UserLogin>,
        web::Data<State>,
    ),
) -> Result<HttpResponse, HttpResponse> {
    let sess: Result<Option<UserIn>, Error> = session.get("uid");
    match sess {
        Ok(Some(user)) => {
            Ok(HttpResponse::Ok()
                .json(user))
        }
        _ => Err(HttpResponse::NotFound()
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
