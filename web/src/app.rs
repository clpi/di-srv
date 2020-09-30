use std::time::Duration;
use serde::{Serialize, Deserialize};
use divdb::db::Db;
use crate::{context::Context, handlers::{index, auth, user, record}};
use actix_web::{
    HttpServer, App, web, HttpRequest, HttpResponse, Responder, dev,
    middleware::Logger, http, get
};
use actix_identity::{Identity, IdentityService, CookieIdentityPolicy};

pub async fn run_api(listener: std::net::TcpListener) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let db = Db::new().await.unwrap();
    let ctx = Context { db };
    HttpServer::new(move ||
        App::new()
            .data(ctx.clone())
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false)))
            .route("/", web::get().to(index))
            .configure(user::routes)
            .configure(auth::routes)
    )
        .listen(listener)?
        .run().await
}

pub fn spawn_api(listener: std::net::TcpListener) -> std::io::Result<dev::Server> {
    let mut sys = actix_rt::System::new("api");
    let db = Db::new();
    let srv = HttpServer::new(||
        App::new()
            .route("/", web::get().to(index))
    )
        .listen(listener)?
        .run();
    Ok(srv)
}


#[derive(Serialize, Deserialize)]
pub struct TestEcho { num: i32, string: String }

#[get("/test")]
pub async fn test_route(req: HttpRequest, test: web::Json<TestEcho>) -> HttpResponse {
    println!("REQ: {:?}", req);
    HttpResponse::Ok().body(&test.string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::{init_service, TestRequest};

    #[actix_rt::test]
    async fn test_route_can_echo() {
        let mut app = init_service(App::new()
            .service(web::resource("/").route(web::post().to(index))),
        );
        let req = TestRequest::get().uri("/")
            .set_json(&TestEcho { num: 7, string: "Test".to_string(), })
            .to_request();
    }

    async fn index_is_accessible() {
    }
}
