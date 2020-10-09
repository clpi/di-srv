pub use divdb::models::User;
use serde::{Serialize, Deserialize};
use actix_identity::{Identity, RequestIdentity};
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};
use futures::future::ready;

#[derive(Serialize, Deserialize)]
pub struct UserIn {
    id: i32,
    email: String,
    username: String,
}

impl From<User> for UserIn {
    fn from(user: User) -> Self { 
        UserIn { id: user.id.unwrap(), email: user.email, username: user.username }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserQuery {
    id: Option<i32>,
    username: Option<String>,
    email: Option<String>
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
