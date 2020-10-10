use std::fs::File;
use divdb::{Db, models::User};
use crate::{models::Response, state::State, handlers::user::*};
use actix_multipart::Multipart;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{Error,
    http::{Cookie, HeaderName, HeaderValue},
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};

pub fn routes() -> actix_web::Scope {
    // -------------/ admin -----------------//
    scope("/upload")
        // ----------------- /admin/db --------------//
        .service(scope("/{uid}")
            .service(
                resource("").route(post().to(upload_user_img))
            )
        )
}

pub async fn upload_user_img(
    data: web::Data<State>, 
    path: web::Path<String>, 
    mut payload: Multipart
    ) -> Result<HttpResponse, Error>
{
    Ok(HttpResponse::Ok().into())
}
