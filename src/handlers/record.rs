use divdb::models::{Model, Record};
use crate::state::State;
use actix_web::{
    http::{Cookie, HeaderName, HeaderValue},
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(scope("rec")
            .service(resource("/{rid}")
                .route(get().to(get_by_id))
            ))
}

pub async fn get_by_id(id: web::Path<i32>, data: web::Data<State>) -> HttpResponse {
    match Record::get_by_id(&data.db, id).await {
        Ok(rec) => HttpResponse::Ok().json(rec),
        Err(_) => HttpResponse::NotFound().json("{}")
    }

}
