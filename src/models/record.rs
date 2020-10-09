pub use divdb::models::User;
use serde::{Serialize, Deserialize};
use actix_identity::{Identity, RequestIdentity};
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};
use futures::future::ready;

#[derive(Serialize, Deserialize)]
pub struct RecordQuery {
    id: Option<i32>,
    username: Option<String>,
    name: Option<String>,
}
