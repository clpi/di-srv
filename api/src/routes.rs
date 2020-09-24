pub mod user;
pub mod record;
pub mod auth;

use crate::context::Context;
use crate::handlers;
//use actix_session::{CookieSession, Session};
use actix_web::{
    web, HttpResponse, HttpRequest, Responder, get, post, Either,
    dev::RequestHead, guard::Guard, http, guard, 
    web::resource as path, 
};

pub struct Route {
    path: String
}

pub async fn index(req: HttpRequest) -> impl Responder {
    println!("{:?}", req);
    HttpResponse::Ok().body("Hello!")
}

pub async fn hello(path: web::Path<String>) -> impl Responder {
    format!("Hello, {}", &path)
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(path("/").to(index));
    cfg.service(path("/hi/{path}").to(hello));
    cfg.service(web::scope("/user/").configure(handlers::user::routes));
    cfg.service(web::scope("/auth/").configure(handlers::auth::routes));
    cfg.service(web::scope("/rec/").configure(handlers::record::routes));
    cfg.service(web::scope("/admin/").guard(guard::Header("content-type", "text/plain")
        ).configure(handlers::admin::routes)
    );
}

pub fn register_route<T>(
    cfg: &mut web::ServiceConfig, 
    route: Vec<&str>, 
    handler: Box<dyn Fn(Context) -> T>) 
    -> Result<(), std::io::Error>
{
    Ok(())
}
