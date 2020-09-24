use std::sync::Mutex;
use crate::{
    context::Context, routes::config_routes,
};
use actix;
use actix_web::{
    web, App, HttpRequest, HttpResponse, HttpServer, Responder, middleware,
    dev::Server, http::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT},
};
use actix_cors::Cors;
use actix_session::{CookieSession, Session};

pub struct Api {
    ctx: Context,
}

impl Api {

    pub async fn run() -> std::io::Result<()> {
        std::env::set_var("RUST_LOG", "actix_web=info");
        let system = actix::System::new("test");
        let addr = "127.0.0.1:7711";
        let ctx = Context::new();
        HttpServer::new(move || {
            let cors = match std::env::var("FRONT_URL").ok() {
                Some(ref url) => Cors::new()
                    .allowed_origin(url),
                None => Cors::new()
                    .allowed_origin("*")
                    .send_wildcard()
            }
                .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION, ACCEPT])
                .allowed_methods(vec!["GET", "POST"])
                .max_age(3600);
            App::new()
                //.data(web::Data::new(ctx))
                .wrap(cors.finish())
                .wrap(middleware::Logger::default())
                .wrap(middleware::Logger::new("%a %{User-Agent}i"))
                .wrap(CookieSession::signed(&[0; 32]).secure(false))
                .service(
                    web::scope("/user")
                        .service(crate::routes::user::get_all)
                )
                .default_service(web::route()
                    .to(|| HttpResponse::NotFound()
                        .content_type("text/plain")
                        .body("Not Found"))
                )
                .configure(config_routes)
        })
            .bind(addr)?
            .run()
            .await
    }

    pub async fn log_init() -> () {
        std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    pub async fn init_db() -> () {
        db::Db::init();
    }

    pub async fn run_dev() {}

    pub async fn run_prod() {}

    pub async fn register_routes() {}
}

pub async fn run() -> () {

}

pub fn run_dev() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| 
        App::new()
    )
        .bind("127.0.0.1:8000")?
        .run();
    Ok(server)
}
