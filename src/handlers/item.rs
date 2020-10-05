use crate::{models::Response, state::State,};
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{
    http::{Cookie, HeaderName, HeaderValue},
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
    .service(scope("/item")
        .service(resource("")
            .route(get().to(|| HttpResponse::Ok().body("")))
        )
    )
    /// ------------ /user/{uid}/item -------- /// [ MAIN /src/handlers/items.rs ]
    .service(scope("/user/{uid}/item")
        .service(resource("")
            .route(get().to(get_user_items))
            .route(put().to(add_item_to_user))
        )
        /// ------------ /user/{uid}/item/{iid} -------- ///
        .service(scope("/{iid}")
            .service(resource("")
                .route(get().to(|| HttpResponse::Ok().body("")))
                .route(put().to(|| HttpResponse::Ok().body("")))
                .route(delete().to(|| HttpResponse::Ok().body("")))
            )
            /// ------------ /user/{uid}/item/{iid}/feed -------- ///
            .service(resource("/feed")
                .route(get().to(|| HttpResponse::Ok().body("")))
        )
        /// ------------ /user/{uid}/item/feed -------- ///
        .service(scope("/feed")
            .service(resource("")
                .route(get().to(|| HttpResponse::Ok().body("")))
            )
        )
    ));
    /// ------------ /user/{uid}/item/feed -------- ///
}

pub async fn get_user_items(id: web::Path<(i32, i32)>, data: web::Data<State>) -> HttpResponse {
    match Record::get_by_id(&data.db, *id).await {
        Ok(rec) => HttpResponse::Ok().json("{}"), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}")
    }

}
