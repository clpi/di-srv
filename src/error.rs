use actix_web::{Error, web, HttpResponse, ResponseError};
use derive_more::Display;
use oxide_auth_actix::WebError;

#[derive(Debug)]
enum AuthError {
    Invalid,
    DoesNotExist,
    Internal,
}

#[derive(Debug)]
pub enum OAuthError {
    AccessFailed,
    NoToken,
    AuthorizationFailed,
    RefreshFailed,
    Invalid(serde_json::Error),
    MissingToken,
    Response(String),
}

