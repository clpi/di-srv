pub mod auth;

use std::time::Duration;
use actix_redis::{RedisActor, RedisSession};
use actix_cors::{Cors, AllOrSome};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    cookie::SameSite,
    http::{self, HeaderName, HeaderValue},
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
        .send_wildcard()
        .max_age(3600)
        .allowed_methods(vec!["GET", "POST", "DELETE", "PUT", "OPTIONS"])
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

pub struct RsSession {}

pub fn redis_session() -> RedisSession {
    RedisSession::new("127.0.0.1:6379", &[0; 32])
        .cookie_http_only(false)
        .cookie_name("r-auth-cookie")
        .cookie_same_site(SameSite::Lax)
}

pub fn session() -> CookieSession {
    CookieSession::signed(&[0; 32])
        .name("auth-session")
        .secure(false)
}

pub fn request_client() -> Client {
    Client::default()
}
