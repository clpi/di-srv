pub mod auth;

use actix_redis::RedisSession;
use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    cookie::SameSite,
    client::Client,
    middleware::{Logger,
        normalize::{NormalizePath, TrailingSlash},
    },
};
use actix_session::CookieSession;

pub fn logger() -> Logger {
    let _log = Logger::new(r#"%a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#);
    Logger::default()
}

pub fn oauth_middleware() {
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
