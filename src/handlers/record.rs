use divdb::models::{Model, Record};
use crate::state::State;
use actix_web::{
    http::{Cookie, HeaderName, HeaderValue},
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
    // ------------ /user/{uid}/record/ -------- /// [ MAIN /src/handlers/record.rs ]
    /*
    .service(scope("/record")
        .service(resource("")
            .route(get().to(get_records))
            .route(post().to(add_record))
        )
        // ------------ /user/{uid}/record/{rid} -------- ///
        .service(scope("/{rid}")
            .service(resource("")
                .route(get().to(get_user_record))
                .route(put().to(update_user_record))
                .route(delete().to(delete_user_record))
            )
            // ------------ /user/{uid}/record/{rid}/items -------- ///
            .service(resource("/items")
                .route(get().to(get_record_items))
                .route(post().to(add_item_to_record))
                .route(put().to(update_item_in_record))
                .route(delete().to(add_item_to_record))
            )
        ) // -- /user/{uid}/record/{rid}
    ); // -- /user/{uid}/record 
    */
        ;

}

pub async fn get_by_id(id: web::Path<i32>, data: web::Data<State>) -> HttpResponse {
    match Record::get_by_id(&data.db, *id).await {
        Ok(rec) => HttpResponse::Ok().json("{}"), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}")
    }

}

pub async fn delete_by_id(id: web::Path<i32>, data: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json("{}")
}
