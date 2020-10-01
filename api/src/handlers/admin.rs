use actix_web::{web::{self, resource, ServiceConfig, scope, put, get, delete, post}, HttpResponse, HttpRequest, http::{HeaderValue, HeaderName, Cookie}};
use actix_identity::{Identity, IdentityService, CookieIdentityPolicy};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(scope("/admin")
            .service(scope("/db")
                .service(resource("/up").route(get().to(db_up)))
                .service(resource("/down").route(get().to(db_down)))
                .service(scope("/{table}")
                    .service(resource("").route(get().to(get_all_table)))
                    .service(resource("/down").route(get().to(table_down)))
                    .service(resource("/up").route(get().to(table_up)))
                ))
            .service(scope("/server")
                .service(resource("").route(get().to(server_info)))
                .service(resource("/up").route(post().to(server_up)))
                .service(resource("/down").route(post().to(server_down)))));
}

pub async fn db_up() -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn db_down() -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn server_info() -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn server_up() -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn server_down() -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn get_all_table(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn table_up(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("")
}

pub async fn table_down(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("")
}
