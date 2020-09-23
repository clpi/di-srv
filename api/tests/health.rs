use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;
//use crate::run_dev;

#[actix_rt::test]
async fn health_check_works(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
