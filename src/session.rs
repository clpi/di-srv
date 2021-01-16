use actix_web::HttpResponse;
use actix_session::Session;
use actix_redis::RedisSession;
use uuid::Uuid;
use jsonwebtoken::TokenData;

pub struct DSession {
    uid: Option<Uuid>,
    jwt: Claim,
}

pub struct Claim {
}

async fn login(uid: Uuid, session: &Session) -> HttpResponse {
    if let Some(uid) = id(session) {
        format!("Already logged in: {}", uid.to_string()).into()
    } else {
        session.set("uid", &uid).unwrap();
        session.renew();
        HttpResponse::Ok().json(true)
    }
}


pub fn id(session: &actix_session::Session) -> Option<uuid::Uuid> {
    match session.get::<Uuid>("uid") {
        Ok(Some(uid)) => Some(uid),
        _ => None,
    }
}

pub fn logout(session: &actix_session::Session) -> HttpResponse {
    if let Some(uid) = id(session) {
        session.purge();
        format!("Logged out: {}", uid.to_string()).into()
    } else {
        "No user to log out".into()
    }
}
