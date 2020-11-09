pub mod admin;
pub mod auth;
pub mod record;
pub mod sse;
pub mod stat;
pub mod user;
pub mod ws;
pub mod item;
pub mod group;
pub mod upload;

use actix_identity::Identity;
use actix_web::{
    App, web, web::ServiceConfig, HttpRequest, HttpResponse, Responder,
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.route("/", web::get().to(static_ind))
        .route("/index", web::get().to(index))
        .service(test_service())
        .service(user::uid_routes())
        .service(user::username_routes())
        .service(auth::routes())
        .service(record::base_routes())
        .service(record::user_record_routes())
        .service(item::base_routes())
        .service(item::user_item_routes())
        .service(admin::routes());
}

pub(crate) fn test_service() ->  actix_web::Resource {
    web::resource("/test/{test}")
        .route(web::get().to(|test: web::Path<String>| {
            HttpResponse::Ok().body(format!("GET /test/{}", test))
        }))
        .route(web::post().to(|| HttpResponse::Ok().body("")))
}

pub(crate) async fn index(id: Identity) -> impl Responder {
    let res = match id.identity() {
        Some(id) => format!("Hello, {}", id),
        None => "Welcome newcomer!".to_string(),
    };
    HttpResponse::Ok().body(res)
}

pub(crate) async fn static_ind(_id: Identity) -> impl Responder {
    //TODO Only works when run in root dir
    let html = String::from_utf8(std::fs::read("static/index.html").unwrap()).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

pub async fn route_404(_req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().body("No route here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::init_service;

    #[actix_rt::test]
    async fn test_route_can_echo() {
        let _app =
            init_service(App::new().service(web::resource("/").route(web::post().to(index))));
    }

    #[actix_rt::test]
    async fn index_get_ok() {
        let _app = init_service(App::new().data(crate::state::state()).configure(routes)).await;
    }
}

