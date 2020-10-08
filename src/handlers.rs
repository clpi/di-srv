pub mod admin;
pub mod auth;
pub mod record;
pub mod sse;
pub mod stat;
pub mod user;
pub mod ws;
pub mod item;
pub mod group;

use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{
    dev, web, web::ServiceConfig, App, HttpRequest, HttpResponse, HttpServer, Responder,
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

pub fn test_service() ->  actix_web::Resource {
    web::resource("/test/{test}")
        .route(web::get().to(|test: web::Path<String>| {
            HttpResponse::Ok().body(format!("GET /test/{}", test))
        }))
        .route(web::post().to(|| HttpResponse::Ok().body("")))
}

pub async fn index(id: Identity) -> impl Responder {
    let res = match id.identity() {
        Some(id) => format!("Hello, {}", id),
        None => "Welcome newcomer!".to_string(),
    };
    HttpResponse::Ok().body(res)
}

pub async fn static_ind(id: Identity) -> impl Responder {
    let html = String::from_utf8(std::fs::read("static/index.html").unwrap()).unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

pub async fn route_404(req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().body("No route here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::{init_service, TestRequest};

    #[actix_rt::test]
    async fn test_route_can_echo() {
        let mut app =
            init_service(App::new().service(web::resource("/").route(web::post().to(index))));
    }

    #[actix_rt::test]
    async fn index_get_ok() {
        let mut app = init_service(App::new().data(crate::state::state()).configure(routes)).await;
    }
}
