use crate::{handlers, middleware, state};
use actix_service::ServiceFactory;
use actix_web::{body, dev, get,  web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::{net::TcpListener, sync::mpsc};

pub async fn run_api(listener: TcpListener) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let srv = HttpServer::new(move || create_app());
    srv.listen(listener)?.run().await?;
    Ok(())
}

pub fn spawn_api(
    listener: TcpListener,
    tx: mpsc::Sender<dev::Server>,
    ) -> std::io::Result<()>
{
    log::info!("Spawning DI server...");
    let mut sys = actix_rt::System::new("api");
    let _db = sys.block_on(divdb::Db::new()).unwrap();
    let srv = HttpServer::new(move || create_app())
        .listen(listener)?
        .run();
    let _ = tx.send(srv.clone());
    sys.block_on(srv)
}


pub fn create_app() -> App<
    impl ServiceFactory<
        Config = (),
        Request = dev::ServiceRequest,
        Response = dev::ServiceResponse<body::Body>,
        Error = Error,
        InitError = (),
    >,
    body::Body,
> {
    App::new()
        .data(state::state().clone())
        .wrap(middleware::cors().finish())
        .wrap(middleware::identity_service())
        //.wrap(middleware::session())
        .wrap(middleware::redis_session())
        .configure(handlers::routes)
}


#[derive(Serialize, Deserialize)]
pub struct TestEcho {
    num: i32,
    string: String,
}

pub async fn test() {

}

#[get("/test")]
pub async fn test_route(req: HttpRequest, test: web::Json<TestEcho>) -> HttpResponse {
    println!("REQ: {:?}", req);
    HttpResponse::Ok().body(&test.string)
}

