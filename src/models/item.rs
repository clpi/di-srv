
pub use divdb::models::User;
use serde::{Serialize, Deserialize};
use actix_identity::{Identity, RequestIdentity};
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};
use futures::future::ready;

#[derive(Serialize, Deserialize)]
pub struct ItemQuery {
    id: Option<String>,
    username: Option<String>,
    name: Option<String>,
    record_name: Option<String>,
}
