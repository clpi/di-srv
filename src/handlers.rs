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

use actix_web::{
    web, web::ServiceConfig, HttpRequest, HttpResponse, Responder, Scope,
};

pub fn routes(base: &str) -> Scope {
    web::scope(base)
        .service(user::routes("/user"))
        .service(record::routes("/record"))
        .service(item::routes("/item"))
        .service(fact::routes("/fact"))
        .service(admin::routes("/admin"))
        .service(auth::routes("/auth"))
}

pub(crate) fn test_service() ->  actix_web::Resource {
    web::resource("/test/{test}")
        .route(web::get().to(|test: web::Path<String>| {
            HttpResponse::Ok().body(format!("GET /test/{}", test))
        }))
        .route(web::post().to(|| HttpResponse::Ok().body("")))
}

pub async fn route_404(_req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().body("No route here")
}

