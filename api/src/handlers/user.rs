use crate::auth::is_logged_in;
use actix_web::{
    get, HttpResponse, web, Responder, guard, HttpRequest,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .guard(guard::default())
            .configure(public::routes)
            .configure(logged_in::routes))
}

pub fn _routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .guard(guard::default())
            .service(web::resource("/{username}"), web::get().to(user::get_user))
            .route(web::resource("name"))
            .route(web::resource("/all"), web::get().to(get_all))
            .route(web::resource("/greet/{name}"), web::get().to(greet)));
}

pub mod public { //unprotected, publicly available
    use super::*;

    pub async fn routes(ctx: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/all"), web::get().to(get_all))
            .service(web::resource("/{username}"), web::get().to(get_user_by_uname))
            .service(web::resource("/{id}"), web::get().to(get_user_by_uid));
    }

    pub async fn get_all() -> impl Responder {
        HttpResponse::Ok().body("all")
    }

    pub async fn get_user_by_uname(username: web::Path<String>) -> impl Responder {
        HttpResponse::Ok().body(format!("{} user::user::get_user()", username))
    }

    pub async fn get_user_by_uid(uid: web::Path<i32>) -> impl Responder {
        HttpResponse::Ok().body(format!("{} user::user::get_user()", uid))
    }
}

pub mod logged_in {
    use super::*;

    pub async fn routes(ctx: &mut web::ServiceConfig) {
        cfg
            .service(web::resource("/ban"), web::get().to(ban_user))
    }

    pub async fn ban_user() - impl Responder {
        HttpResponse::Ok().body("private route")
    }
}

//#[get("/all")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body("all")
}

pub async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok()
        .body(format!("Hello, {}", name))
}

pub fn check_token_exists(req: HttpRequest) -> bool {
    req.headers().contains_key("token")
}
