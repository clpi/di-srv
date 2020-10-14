use actix_web::client::Client;
use div_cloud::auth::{cognito::CognitoClient, types::*};
use actix_session::{Session, SessionStatus};
use serde::{Serialize, Deserialize};
use crate::{state::State, models::UserIn};
use actix_identity::Identity;
use actix_web::{ Error, cookie::Cookie,
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse, Scope,
};
use com::auth::Auth;
use divdb::models::user::*;

#[derive(Serialize, Deserialize)]
pub struct CognitoIn {}

pub fn routes() -> Scope {
    scope("/auth")
        .service(cognito_routes())
        .service(resource("/login")
            .route(get().to(|| HttpResponse::Ok().body("GET /auth/login")))
            .route(post().to(cognito_login_user)),
        )
        .service(resource("/signup")
            .route(get().to(|| HttpResponse::Ok().body("GET /auth/signup")))
            .route(post().to(cognito_signup)),
        )
        .service(resource("/logout")
            .route(get().to(logout_session))
            .route(post().to(cognito_logout_user)),
        )
        .service(resource("/refresh")
            .route(get().to(check_id))
            .route(post().to(refresh_login)),
        )
        .service(resource("/check")
            .route(get().to(check_session))
        )
}

pub fn cognito_routes() -> Scope {
        scope("/cognito")
            //.wrap_fn(validate)
            .service(resource("/login").route(post().to(cognito_login_user)))
            .service(resource("/logout").route(post().to(cognito_logout_user)))
            .service(resource("/signup").route(post().to(cognito_signup)))
            .service(resource("/create").route(post().to(cognito_create_user)))
            .service(resource("/authorize").route(post().to(cognito_authorize)))
            .service(resource("/token").route(post().to(cognito_token)))
            .service(scope("/{username}")
                .service(resource("")
                    .route(get().to(cognito_get_user))
                    .route(delete().to(cognito_delete_user))
                )
                .service(resource("/confirm")
                    .route(post().to(cognito_confirm_signup))
                )
            )
}

pub fn validate(session: &Session) -> Result<UserIn, actix_web::HttpResponse> {
    let user: Option<UserIn> = session.get("uid").unwrap_or(None);
    match user {
        Some(user) => { session.renew(); Ok(user) },
        None => Err(HttpResponse::Unauthorized().json("Unauthorized"))
    }
}

pub fn validate_id(id: &Identity) -> Result<UserIn, actix_web::HttpResponse> {
    let user: Option<String> = id.identity();
    match user {
        Some(user) => { Ok(serde_json::from_str(&user).unwrap()) },
        None => Err(HttpResponse::Unauthorized().json("Unauthorized"))
    }
}

pub async fn logout_session(id: Identity, session: Session) -> Result<HttpResponse, HttpResponse> {
    let sess: Result<Option<UserIn>, Error> = session.get("uid");
    match sess {
        Ok(Some(user)) => { 
            id.forget();
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
    (id,  data): (Identity, web::Data<State>)) -> HttpResponse 
{
    match id.identity() {
        Some(id) => {
            println!("REFRESH: {}", id);
            let user: UserIn = serde_json::from_str(&id).unwrap();
            HttpResponse::Ok()
                .set_header("authorization", "true")
                .json(&user)
        },
        None => HttpResponse::Gone()
            .set_header("authorization", "false")
            .json(false)
        
    }
}

pub async fn check_id(
    (id, req, user, data): (
        Identity,
        HttpRequest,
        web::Json<UserLogin>,
        web::Data<State>,
    ),
) -> HttpResponse {
    match id.identity() {
        Some(id) => {
            let user: UserIn = serde_json::from_str(&id).unwrap();
            HttpResponse::Ok()
                .set_header("authorization", "true")
                .json(&user)
        }
        None => HttpResponse::NotFound()
            .set_header("authorization", "false")
            .json(false)
    }
}

pub async fn check_session(
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

pub async fn cognito_authorize(
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

pub async fn cognito_get_user(
    (req,  data, username): (HttpRequest,web::Data<State>, web::Path<String>) ) -> HttpResponse 
{
    match &data.cognito.get_user(username.into_inner().as_str()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn cognito_confirm_signup(
    (req,  data, username): (HttpRequest,web::Data<State>, web::Path<String>) ) -> HttpResponse 
{
    match &data.cognito.confirm_signup(username.into_inner()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn cognito_delete_user(
    (req,  data, username): (HttpRequest,web::Data<State>, web::Path<String>) ) -> HttpResponse 
{
    match &data.cognito.delete_user(username.into_inner()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn cognito_create_user(
    (req,  data, user): (HttpRequest,web::Data<State>, web::Json<CgUserSignup>) ) -> HttpResponse 
{
    match &data.cognito.create_user(user.into_inner(), true).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn cognito_signup(
    (req,  data, user): (HttpRequest,web::Data<State>, web::Json<CgUserSignup>) ) -> HttpResponse 
{
    match &data.cognito.signup_user(user.into_inner()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn cognito_login_user(
    (data, user): (web::Data<State>, web::Json<CgUserLogin>) ) -> HttpResponse 
{
    match &data.cognito.login_user(user.into_inner()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn cognito_logout_user(
    (req,  data, body): (HttpRequest,web::Data<State>, web::Json<String>) ) -> HttpResponse 
{
    match &data.cognito.signout_user(body.into_inner()).await {
        Ok(res) => HttpResponse::Ok()
            .content_type("application/json")
            .json(res),
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn cognito_token(
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
