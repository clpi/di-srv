// IO.DIV.IS - div.is backend API and server
//
// TODO: Start scaffolding a basic HTML web interface frontend to interact and implement
//       frontend functionality for crud operations in the backend, as well as test
//       experimental functionality (websockers, sse, etc.) and begin the incremental
//       design process for a full featured frontend
//
// TODO: Start scaffolding flutter app (dimo) with basic functionality and communication
//       with io.div.is backend, figure out ci/cd deployment pipeline infra
//
// TODO: Deploy to io.div.is and create ansible playbook to automate process (multi-stage
//       docker build -> create binary w/o rust toolchain) to deploy to io.div.is, along
//       side scaffolded frontend web interface
//

use crate::{handlers, middleware, state};
use actix_service::ServiceFactory;
use actix_session::{Session, UserSession};
use actix_web::*;
use actix_web::{body, dev, get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::{net::TcpListener, sync::mpsc};
use tokio::*;

pub async fn run_api(listener: TcpListener) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let srv = HttpServer::new(move || create_app());
    srv.listen(listener)?.run().await?;
    Ok(())
}

pub fn spawn_api(listener: TcpListener, tx: mpsc::Sender<dev::Server>) -> std::io::Result<()> {
    let mut sys = actix_rt::System::new("api");
    let db = sys.block_on(divdb::Db::new()).unwrap();
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

#[get("/test")]
pub async fn test_route(req: HttpRequest, test: web::Json<TestEcho>) -> HttpResponse {
    println!("REQ: {:?}", req);
    HttpResponse::Ok().body(&test.string)
}

