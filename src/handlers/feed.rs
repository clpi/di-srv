use actix_web::{Scope,
    get, post, put, delete,
    web::{self, scope, ServiceConfig},
    HttpResponse, HttpRequest,
};

pub fn routes(base: &str) -> actix_web::Scope {
    scope(base)
}
