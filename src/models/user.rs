pub use divdb::models::{Record, User, Item};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use actix_identity::{Identity, RequestIdentity};
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};
use futures::future::ready;

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

pub struct UserData {
    id: Uuid,
    username: String,
    email: String,
    records: Vec<Record>,
    items: Vec<Item>,
    created_at: DateTime<Utc>,
}

/*
impl FromRequest for UserIn {
    type Error = Error;
    type Config = ();
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let id = RequestIdentity::get_identity(req);
    }
}
*/
