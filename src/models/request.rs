use serde::{Serialize, Deserialize};
use crate::state::State;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{
    http::{Cookie, HeaderName, HeaderValue},
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};
use com::auth::Auth;
use divdb::models::{Model, user::*};

pub struct AuthRequest<T: Serialize> {
    id: Identity,
    req: HttpRequest,
    data: web::Data<State>,
    body: web::Json<T>,
}

pub struct UserRequest;

pub struct IdQueryParam<T: Model + Serialize> {
    id: i32,
    model: T,
}

/*
#[derive(FromRequest)]
pub struct ReqData<T> {
    path: web::Path<(String, String)>,
    query: web::Query<HashMap<String, String>>,
    json: web::Json<T>,
}
*/
