use actix_web::{web, App, HttpRequest, HttpServer, Responder, dev::Server};
use diva::app::Api;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    Api::run().await
    /*
    HttpServer::new(|| {
        App::new()
    })
    .bind("127.0.0.1:7788")?
    .run()
    .await
    */
}
