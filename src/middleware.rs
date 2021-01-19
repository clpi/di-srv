use actix_redis::RedisSession;
use actix_cors::Cors;
use actix_web::{
    cookie::SameSite,
    client::Client,
    middleware::{Logger,
        normalize::{NormalizePath, TrailingSlash},
    },
};
use actix_session::CookieSession;
// use actix_web_httpauth::{
//     middleware::HttpAuthentication,
//     extractors::bearer::BearerAuth
// };

// pub fn auth() -> HttpAuthentication {
//     HttpAuthentication::bearer

// }

pub fn logger() -> Logger {
    let _log = Logger::new(r#"%a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#);
        // .custom_request_replace("JWT_ID", |req| parse_jwt_id(req.headers().get("Authorization")));
    Logger::default()
}

// pub fn cognito() -> std::sync::Arc<CognitoValidator> {
//     let cognito_validator =
//         std::sync::Arc::new(CognitoValidator::create().expect("Cognito configuration error"));
//     cognito_validator
// }

pub fn oauth_middleware() {
}

pub fn cors() -> Cors {
    Cors::default()
        .send_wildcard()
        .max_age(3600)
        .allowed_methods(vec!["GET", "POST", "DELETE", "PUT", "OPTIONS"])
}

pub fn trim_trailing_slash() -> NormalizePath {
    NormalizePath::new(TrailingSlash::Trim)
}

pub struct RsSession {}

pub fn redis_session(key: &Option<String>) -> RedisSession {
    let rk = if let Some(k) = key { k.as_bytes() } else { &[0; 32] };
    RedisSession::new("127.0.0.1:6379", &rk)
        .cookie_http_only(false)
        .cookie_name("r-auth-cookie")
        .cookie_same_site(SameSite::Lax)
}

pub fn session(key: &Option<String>) -> CookieSession {
    let rk = if let Some(k) = key { k.as_bytes() } else { &[0; 32] };
    CookieSession::signed(&rk)
        .name("auth-session")
        .secure(false)
}

pub fn request_client() -> Client {
    Client::default()
}
