pub mod user;
pub mod record;
pub mod auth;

use crate::context::Context;
use crate::handlers;
//use actix_session::{CookieSession, Session};
use actix_web::{
    web, HttpResponse, HttpRequest, Responder, get, post, Either,
    dev::RequestHead, guard::Guard, http, guard,
};

pub struct Route {
    path: String
}

fn test(_ctx: web::Data<Context>, _req: HttpRequest) -> &'static str {
    "Hello world!"
}

#[get("/")]
pub async fn index(req: HttpRequest) -> impl Responder {
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

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(index))
        .service(web::scope("/user/")
            .route("/all", web::get().to(handlers::user::get_all))
            .route("/greet/{name}", web::get().to(handlers::user::greet))
        )
        .service(web::scope("/u/").configure(handlers::user::routes))
        .service(web::scope("/auth/").configure(handlers::auth::routes)
        .service(web::scope("/admin/")
            .guard(guard::Header("content-type", "text/plain")
        .service(web::scope("/rec/")
        .service(web::scope("r")
            .route("/test", web::get().to(|| HttpResponse::Ok().body("")))
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
