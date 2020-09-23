pub mod user;
pub mod record;
pub mod auth;

use crate::context::Context;
//use actix_session::{CookieSession, Session};
use actix_web::{
    web, HttpResponse, HttpRequest, Responder, get, post, Either,
    dev::RequestHead, guard::Guard, http,
};

pub struct Route {
    path: String
}

fn test(_ctx: web::Data<Context>, _req: HttpRequest) -> &'static str {
    "Hello world!"
}

#[get("/")]
pub async fn index(data: web::Data<Context>, req: HttpRequest) -> impl Responder {
    println!("{:?}", req);
    HttpResponse::Ok().body("Hello!")
}

#[get("/d/{path}")]
pub async fn hello(path: web::Path<String>) -> impl Responder {
    format!("Hello, {}", &path)
}

#[get("/greet")]
pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/test")
        .route(web::get().to(|| HttpResponse::Ok()))
        .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
    )
        .service(index)
        .service(hello)
        .service(greet);
}

pub fn register_route<T>(
    cfg: &mut web::ServiceConfig, 
    route: Vec<&str>, 
    handler: Box<dyn Fn(Context) -> T>) 
    -> Result<(), std::io::Error>
{
    Ok(())
}
