use actix_web::client::Client;
use div_cloud::cognito::types::*;
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
        //.wrap_fn(validate)
        .route("/login", post().to(login_user))
        .route("/logout", post().to(logout_user))
        .route("/signup", post().to(signup_user))
        .route("/authorize", post().to(authorize_user))
        .route("/token", post().to(get_token))
        // NOTE non-cognito routes
        .route("/signin", post().to(signin_user))
        .route("/register", post().to(register_user))
        // ADMIN ROUTES -- wrap_fn(validate_admin)
        .service(resource("")
            .route(post().to(create_user))
            .route(get().to(get_users))
        )
        .service(scope("/{username}")
            .service(resource("")
                .route(get().to(get_user))
                .route(delete().to(delete_user))
            )
            .service(resource("/confirm")
                .route(post().to(confirm_signup))
            )
            .service(resource("/{attribute}")
                .route(get().to(get_attribute))
                .route(post().to(add_attribute))
                .route(delete().to(delete_attribute))
                .route(put().to(set_attribute))
            )
        )
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

pub fn ext_cognito_routes() -> Scope {
    scope("/cognito")
        .service(resource("/authorize").route(post().to(cognito_authorize)))
        .service(resource("/token").route(post().to(cognito_token)))
        .service(resource("/userinfo").route(post().to(cognito_userinfo)))
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

pub async fn refresh_login(
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

pub async fn check_auth() {}



pub async fn authorize_user(
    (req,  data, body): (HttpRequest,web::Data<State>, web::Json<CognitoIn>) ) -> HttpResponse
{
    let payload = body.into_inner();
    match Client::new()
        .get("https://in.div.is/oauth2/authorize")
        .send_json(&payload).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(true),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn get_user(
    (req,  data, username): (HttpRequest,web::Data<State>, web::Path<String>) ) -> HttpResponse
{
    match &data.cognito.get_user(username.into_inner().as_str()).await {
        Ok(res) => {
            log::info!("user: {}", res.username);
            HttpResponse::Ok()
                .content_type("application/json")
                .json(res)
        },
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn confirm_signup(
    (req,  data, username): (HttpRequest,web::Data<State>, web::Path<String>) ) -> HttpResponse
{
    match &data.cognito.confirm_signup(username.into_inner()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn delete_user(
    (req,  data, username): (HttpRequest,web::Data<State>, web::Path<String>) ) -> HttpResponse
{
    match &data.cognito.delete_user(username.into_inner()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn create_user(
    (req,  data, user): (HttpRequest,web::Data<State>, web::Json<CgUserSignup>) ) -> HttpResponse
{
    match &data.cognito.create_user(user.into_inner(), true).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn signup_user(
    (req,  data, user): (HttpRequest,web::Data<State>, web::Json<CgUserSignup>) ) -> HttpResponse
{
    match &data.cognito.signup_user(user.into_inner()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn login_user(
    (data, user): (web::Data<State>, web::Json<CgUserLogin>) ) -> HttpResponse
{
    match &data.cognito.login_user(user.into_inner()).await{
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn logout_user(
    (req,  data, body): (HttpRequest,web::Data<State>, web::Json<String>) ) -> HttpResponse
{
    match &data.cognito.signout_user(body.into_inner()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn get_token(
    (req,  data, body): (HttpRequest,web::Data<State>, web::Json<CognitoIn>), ) -> HttpResponse
{
    let payload = body.into_inner();
    let res = Client::new()
        .post("https://in.div.is/oauth2/token")
        .send_json(&payload)
        .await;
    match res {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(true),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn get_users(
    (req,  data, username): (HttpRequest,web::Data<State>, web::Path<String>) ) -> HttpResponse
{
    match &data.cognito.get_user(username.into_inner().as_str()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}


pub async fn set_attribute(
    (req,  data, username): (HttpRequest,web::Data<State>, web::Path<String>) ) -> HttpResponse
{
    match &data.cognito.get_user(username.into_inner().as_str()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn get_attribute(
    (req,  data, username): (HttpRequest,web::Data<State>, web::Path<String>) ) -> HttpResponse
{
    match &data.cognito.get_user(username.into_inner().as_str()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn add_attribute(
    (req,  data, username): (HttpRequest,web::Data<State>, web::Path<String>) ) -> HttpResponse
{
    match &data.cognito.get_user(username.into_inner().as_str()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}


pub async fn delete_attribute(
    (req,  data, username): (HttpRequest,web::Data<State>, web::Path<String>) ) -> HttpResponse
{
    match &data.cognito.get_user(username.into_inner().as_str()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}


pub async fn cognito_authorize() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn cognito_token() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn cognito_userinfo() -> HttpResponse {
    HttpResponse::Ok().finish()
}
