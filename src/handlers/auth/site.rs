use actix_session::Session;
use crate::{state::State, models::UserIn};
use actix_web::{ Error,
    web::{self, delete, get, post, put, resource, scope},
    HttpRequest, HttpResponse, Scope,
};
use div_db::models::user::*;

pub fn routes(base: &str) -> Scope {
    scope(base)
        .route("/signin", post().to(signin_user))
        .route("/register", post().to(register_user))
        .route("/refresh", post().to(check_session_with_user))
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

