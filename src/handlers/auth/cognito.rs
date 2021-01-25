use actix_web::client::Client;
use div_cloud::cognito::types::*;
use crate::state::State;
use serde::{Serialize, Deserialize};
use actix_web::{
    web::{self, delete, get, post, put, resource, scope},
    HttpRequest, HttpResponse, Scope,
};

#[derive(Serialize, Deserialize)]
pub struct CognitoIn {
    pub username: String,
    pub password: String,
}

pub fn routes(base: &str) -> actix_web::Scope {
    scope(base)
        .route("", post().to(create_user))
        .route("", get().to(get_users))
        .route("/login", post().to(login_user))
        .route("/logout", post().to(logout_user))
        .route("/signup", post().to(signup_user))
        .route("/authorize", post().to(authorize_user))
        .route("/token", post().to(get_token))
        .service(cognito_user("/{username}"))
}

pub fn cognito_user(base: &str) -> actix_web::Scope {
    scope(base)
        .route("", get().to(get_user))
        .route("", delete().to(delete_user))
        .route("/confirm", post().to(confirm_signup))
        .service(self::cognito_user_attributes("/{attrib}"))
}

pub fn cognito_user_attributes(base: &str) -> actix_web::Resource {
    web::resource(base)
        .route(get().to(get_attribute))
        .route(post().to(add_attribute))
        .route(delete().to(delete_attribute))
        .route(put().to(set_attribute))
}

pub fn ext_cognito_routes() -> Scope {
    scope("/ext")
        .service(resource("/authorize").route(post().to(cognito_authorize)))
        .service(resource("/token").route(post().to(cognito_token)))
        .service(resource("/userinfo").route(post().to(cognito_userinfo)))
}

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
