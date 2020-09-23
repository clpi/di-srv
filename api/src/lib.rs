use actix_web::{
    web, App, HttpRequest, HttpResponse, HttpServer, Responder,
    dev::Server,
};

pub fn run_dev() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();
    Ok(server)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

