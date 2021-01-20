pub use div_db::models::{Record, User, Item};
use actix_web::{Responder, HttpResponse, HttpRequest};
use futures::future::{ready, Ready};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserIn {
    id: Uuid,
    email: String,
    username: String,
}

impl From<User> for UserIn {
    fn from(user: User) -> Self {
        UserIn { id: user.id, email: user.email, username: user.username }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserQuery {
    id: Option<Uuid>,
    username: Option<String>,
    email: Option<String>
}

pub struct Resp<'de, T: Deserialize<'de> + Serialize + div_db::models::Model>(&'de T);

impl<'de, T> actix_web::Responder for Resp<'de, T>
where
    T: div_db::models::Model + Deserialize<'de> + Serialize {
    type Error = actix_web::Error;
    type Future = Ready<Result<HttpResponse, actix_web::Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(&self.0)))

    }
}
