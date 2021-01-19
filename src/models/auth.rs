use actix_session::Session;
use div_db::models::User;
use actix_web::{
    web, dev::Payload, FromRequest, HttpRequest,
};


pub struct CognitoRequest {
    pub sub: Option<String>,
    pub token: Option<String>,
}
pub struct AuthRequest {

}

impl CognitoRequest {

    pub async fn get_user(token: String) -> () {}
}
