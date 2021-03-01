pub mod admin;
pub mod graphql;
pub mod auth;
pub mod record;
pub mod user;
pub mod item;
pub mod group;
pub mod upload;
pub mod public;
pub mod fact;
pub mod feed;

use crate::state::State;
use serde::{Deserialize, Serialize};
use actix_web::{
    web, web::ServiceConfig, HttpRequest, HttpResponse, Responder,
};

pub mod api {

    use super::*;

    pub fn routes(base: &str) -> actix_web::Scope {
        web::scope(base)
            .service(user::routes("/user"))
            .service(record::routes("/record"))
            .service(item::routes("/item"))
            .service(fact::routes("/fact"))
            .service(admin::routes("/admin"))
            .service(auth::routes("/auth"))
            .service(feed::routes("/feed"))
    }
}

pub(crate) fn _test_srv() ->  actix_web::Resource {
    web::resource("/test/{test}")
        .route(web::get().to(|test: web::Path<String>| {
            HttpResponse::Ok().body(format!("GET /test/{}", test))
        }))
        .route(web::post().to(|| HttpResponse::Ok().body("")))
}

pub async fn route_404(_req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().body("No route here")
}

#[async_trait::async_trait]
pub trait Crud<'de>: Deserialize<'de>  {
    async fn create(data: web::Data<State>, m: web::Json<Self>) -> actix_web::Result<HttpResponse>;

    async fn get(data: web::Data<State>, id: web::Path<uuid::Uuid>) -> actix_web::Result<HttpResponse>;

    async fn delete(data: web::Data<State>, id: web::Path<uuid::Uuid>) -> actix_web::Result<HttpResponse>;

    async fn update(data: web::Data<State>, m: web::Json<Self>) -> actix_web::Result<HttpResponse>;

}

// pub async fn crud_id<M: div_db::models::Model>(path: &str) -> actix_web::Resource {
//     web::resource(path)
//         .route(web::get().to(|data: web::Data<State>, id: web::Path<uuid::Uuid>| async {
//             let db = data.db.clone();
//             let res = M::fetch_from_id(&db.lock().unwrap(), id.into_inner()).await.unwrap();
//             HttpResponse::Ok().json(&res)
//         }))
//         .route(web::post().to(move |data: web::Data<State>, id: web::Path<uuid::Uuid>| async {
//             let db = data.db.lock().unwrap();
//             let res = M::fetch_from_id(&db, id.into_inner()).await.unwrap(); //FIX
//             HttpResponse::Ok().json(&res)
//         }))
//         .route(web::delete().to(move |data: web::Data<State>, id: web::Path<uuid::Uuid>| async {
//             let db = data.db.lock().unwrap();
//             let res = M::delete_from_id(&db, id.into_inner()).await.unwrap(); //FIX
//             HttpResponse::Ok().json(&res)
//         }))
// }
