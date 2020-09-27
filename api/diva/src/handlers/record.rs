use crate::auth::verify;
use actix_web::{
    get, HttpResponse, web, Responder, guard
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    use self::{user, admin};
    cfg.service(
        web::scope("/rec/")
            .scope("/user/")
                .guard(guard::default())
                .route(web::resource("/all"), web::get().to(user::all))
                .service(web::resource("/all"), web::get().to(user::all))
                .route(web::resource("/add"), web::get().to(user::delete))
                .service(web::resource("/delete"), web::delete().to(user::delete))
            .scope("/admin/")
                .guard(guard::default())
                .route(web::get().to(admin::all))
                .service(web::get().to(admin::clear)));
}

pub mod user {
    use super::*;

    pub async fn all() -> impl Responder {
        HttpResponse::Ok().body("user::all")
    }

    pub async fn add() -> impl Responder {
        HttpResponse::Ok().body("all")
    }

    pub async fn delete() -> impl Responder {
        HttpResponse::Ok().body("all")
    }
}


pub mod admin {
    use super::*;

    #[get("/all")]
    pub async fn clear() -> impl Responder {
        HttpResponse::Ok().body("")
    }

    #[get("/all")]
    pub async fn archive() -> impl Responder {
        HttpResponse::Ok().body("")
    }
}

pub mod add {}
