pub mod user;
pub mod auth;
pub mod record;

use actix_web::{
    HttpServer, App, web, HttpRequest, HttpResponse, Responder, dev,
    web::ServiceConfig,
};
use actix_identity::{Identity, IdentityService, CookieIdentityPolicy};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .route("/", web::get().to(static_index))
        .route("/index", web::get().to(index));
}

pub async fn index(id: Identity) -> impl Responder {
    let res = match id.identity() {
        Some(id) => format!("Hello, {}", id),
        None => "Welcome newcomer!".to_string()
    };
    HttpResponse::Ok().body(res)
}

pub async fn static_index(id: Identity) -> impl Responder {
    let html = r#"<html>
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
