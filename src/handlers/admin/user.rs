use actix_web::{ Error,
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};

pub fn routes(base: &str) -> actix_web::Scope {
    scope("/user")
        .service(uid_routes("/{uid}"))
}

pub fn uid_routes(base: &str) -> actix_web::Scope {
    scope(base)
        .route("", delete().to(delete_user_by_id))
        .route("", post().to(|| HttpResponse::Ok().finish()))

}

pub async fn delete_user_by_id(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body("")
}
