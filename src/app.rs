use std::{net::TcpListener,  sync::mpsc};
use serde::{Serialize, Deserialize};
use actix_service::ServiceFactory;
use divdb::db::Db;
use actix_cors::Cors;
use crate::{
    state::{self, State}, 
    handlers::{self,  index, auth, user, record, admin}, middleware
};
use actix_web::{
    HttpServer, App, web, HttpRequest, HttpResponse, Responder, dev,
    middleware::{Logger, DefaultHeaders}, http, get,
    body, Error,
};
use actix_identity::{IdentityService, CookieIdentityPolicy};

pub async fn run_api(listener: TcpListener) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let srv = HttpServer::new(move ||
        App::new()
            .data(state::state())
            .wrap(middleware::logger())
            .wrap(middleware::identity_service())
            .configure(handlers::routes)
    );
    srv
        .listen(listener)?
        .run().await?;
    Ok(())
}

pub fn spawn_api(listener: TcpListener, tx: mpsc::Sender<dev::Server>) -> std::io::Result<()> {
    let mut sys = actix_rt::System::new("api");
    let srv = HttpServer::new(move ||
        App::new()
            .data(state::state())
            .wrap(middleware::logger())
            .wrap(middleware::identity_service())
            .configure(handlers::routes)
    )
        .listen(listener)?
        .run();
    let _ = tx.send(srv.clone());
    sys.block_on(srv)
}

pub fn create_app(state: &State) -> App<
    impl ServiceFactory<
        Config = (),
        Request = dev::ServiceRequest,
        Response = dev::ServiceResponse<body::Body>,
        Error = Error,
        InitError = (),
    >, body::Body> {
        App::new()
            .data(state::state())
            .wrap(middleware::cors().finish())
            .wrap(middleware::identity_service())
            .configure(handlers::routes)
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
    use actix_web::test::{self, init_service};

    #[actix_rt::test]
    async fn identity_service_works() {

    }
}
