use std::sync::Arc;
use divdb::db::Db;
use crate::context::Context;
use actix_web::{
    HttpServer, App, web, HttpRequest, HttpResponse, Responder,
    dev::Server,
};

pub async fn run_api(listener: std::net::TcpListener) -> std::io::Result<()> {
    let db = Db::new().await.unwrap();
    let ctx = Context { db };
    HttpServer::new(move ||
        App::new()
            .data(ctx.clone())
            .route("/", web::get().to(index))
            .configure(crate::routes::routes)
    )
        .listen(listener)?
        .run().await
}

pub fn run(listener: std::net::TcpListener) -> std::io::Result<Server> {
    let srv = HttpServer::new(||
        App::new()
            .route("/", web::get().to(index))
    )
        .listen(listener)?
        .run();
    Ok(srv)
}

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn index_accessible() {
        let res = index().await;
        assert!(res.status().is_success())
    }
}
