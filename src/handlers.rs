pub mod admin;
pub mod auth;
pub mod record;
pub mod user;
pub mod item;
pub mod group;
pub mod upload;
pub mod public;

use actix_session::Session;
use actix_web::{ App,
    web, web::ServiceConfig, HttpRequest, HttpResponse, Responder,
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(test_service())
        .service(public::public_routes())
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

pub(crate) async fn index(id: Session) -> impl Responder {
    let res = match id.get::<String>("id") {
        Ok(Some(id)) => format!("Hello, {}", &id),
        Ok(None) => "Welcome!".to_string(),
        Err(_) => "Error".to_string(),
    };
    HttpResponse::Ok().body(res)
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

}

