pub mod auth;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    client::Client, 
    middleware::{Logger, 
        normalize::{NormalizePath, TrailingSlash},
    },
};
use actix_session::{Session, UserSession, CookieSession};

pub fn logger() -> Logger {
    Logger::new(r#"%a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#)
}

pub fn cors() -> Cors {
    Cors::new()
        .allowed_origin("*")
        .max_age(3600)
        .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
        .send_wildcard()
}

pub fn trim_trailing_slash() -> NormalizePath {
    NormalizePath::new(TrailingSlash::Trim)
}

pub fn identity_service() -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(&[0; 32])
            .name("auth-cookie")
            .secure(false),
    )
}

pub fn session() -> CookieSession {
    CookieSession::signed(&[0; 32])
        .name("auth-session")
        .secure(false)
}

pub fn request_client() -> Client {
    Client::default()
}
