use crate::state::State;
use actix_web::{
    http::{Cookie, HeaderName, HeaderValue},
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("/stats").service(resource("/{uid}").route(get().to(get_user_stats))));
}

pub async fn get_user_stats(data: web::Data<State>, req: HttpRequest) -> HttpResponse {
    let request = req.query_string();
    HttpResponse::Ok().body("get_user_stats")
}
