pub mod user;
pub mod auth;
pub mod record;
pub mod admin;
pub mod ws;
pub mod sse;

use actix_web::{
    HttpServer, App, web, HttpRequest, HttpResponse, Responder, dev,
    web::ServiceConfig,
};
use actix_files::Files;
use actix_identity::{Identity, IdentityService, CookieIdentityPolicy};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .route("/", web::get().to(static_ind))
        .route("/index", web::get().to(index));
    user::routes(cfg);
    auth::routes(cfg);
    record::routes(cfg);
    admin::routes(cfg);
}

pub async fn index(id: Identity) -> impl Responder {
    let res = match id.identity() {
        Some(id) => format!("Hello, {}", id),
        None => "Welcome newcomer!".to_string()
    };
    HttpResponse::Ok().body(res)
}

pub async fn static_ind(id: Identity) -> impl Responder {
    let html = String::from_utf8(std::fs::read("static/index.html").unwrap()).unwrap();
    let htm = r#"<html>
        <head><title>div.is api</title></head>
        <body>
            <h1>div.is api</h1>
            <h3>welcome</h1>
            <form target="/" method="post" enctype="multipart/form-data" id="myForm" >
                <input type="text"  id="text" name="text" value="test_text"/>    
                <input type="number"  id="number" name="number" value="123123"/>    
                <input type="button" value="Submit" onclick="myFunction()"></button>
            </form>
            <input type="file" multiple name="file" id="myFile"/>
        </body>
    </html>"#;
    HttpResponse::Ok().body(html)
}

pub async fn route_404(req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().body("No route here")
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
    }

    #[actix_rt::test]
    async fn index_get_ok() {
        let mut app = init_service(App::new()
            .data(crate::state::state())
            .configure(routes)).await;
    }
}
