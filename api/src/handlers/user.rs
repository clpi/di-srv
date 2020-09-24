use crate::auth::is_logged_in;
use actix_web::{
    get, HttpResponse, web, Responder, guard, HttpRequest
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .configure(public::routes));
            .guard(
                guard::All(
                    guard::Any(
                        guard::Get() || guard::Post()
                    ) && guard::Header("user", "username")
    cfg.service(web::scope("/op").configure(private::routes));
    cfg.service(
        .configure(public::routes)
        .configure(logged_in::routes)
        .guard(guard::fn_guard(|req| true));
}

pub fn _routes(cfg: &mut web::ServiceConfig) {
    cfg.service()
        web::scope("/user/")
            .service(web::resource("/{username}").to(user::get_user_by_uname))
            .route(web::resource("/{id}"), web::get().to(user::get_user_by_id))
            .route(web::resource("/all"), web::get().to(get_all))
            .route(web::resource("/greet/{name}"), web::get().to(greet)));
}

pub mod public { //unprotected, publicly available
    use super::*;

    pub fn routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/all"), web::get().to(get_all))
            .service(get_all)
            .service(get_user_by_uname)
            .service(get_user_by_uid);
    }

    #[get("/all")]
    pub  fn get_all() -> HttpResponse {
        //HttpResponse::Ok().body("all")
            "hello".to_string()
    }

    #[get("/{username}")]
    pub  fn get_user_by_uname(username: web::Path<String>) -> impl Responder {
        HttpResponse::Ok().body(format!("{} user::user::get_user()", username))
    }

    #[get("/{uid}")]
    pub  fn get_user_by_uid(uid: web::Path<i32>) -> impl Responder {
        HttpResponse::Ok().body(format!("{} user::user::get_user()", uid))
    }
}

pub mod logged_in {
    use super::*;

    pub  fn routes(cfg: &mut web::ServiceConfig) {
        cfg.route(web::route().to)
        cfg.service(web::resource("/ban").to(ban_user));
    }

    pub  fn ban_user() -> impl Responder {
        HttpResponse::Ok().body("private route")
    }
}

//#[get("/all")]
pub  fn get_all() -> impl Responder {
    HttpResponse::Ok().body("all")
}

pub  fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok()
        .body(format!("Hello, {}", name))
}

pub fn check_token_exists(req: HttpRequest) -> bool {
    req.headers().contains_key("token")
}

pub fn logged_in_guard() -> impl guard::Guard { 
    guard(guard::All(
        guard::Any(
            guard::Get() || guard::Post()
        ) && (
            guard::Header("user", "username")
        )
    ))

}
