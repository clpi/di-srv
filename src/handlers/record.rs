use divdb::models::{Model, Record, User, Item};
use crate::state::State;
use actix_identity::Identity;
use actix_web::{ Scope,
    http::{Cookie, HeaderName, HeaderValue},
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};
pub fn base_routes() -> Scope {
    scope("/record")
        .service(resource("")
            .route(get().to(|| HttpResponse::Ok().body("")))
        )
        .service(scope("/{rid}")
            .service(resource("")
                .route(get().to(get_record_by_id))
                .route(delete().to(delete_record_by_id))
            )
        )
}

pub fn user_record_routes() -> Scope {
    scope("/user/{uid}")
        // ------------ /user/{uid}/record/ -------- /// 
        .service(scope("/records")
            .service(resource("")
                .route(get().to(get_user_records))
                .route(post().to(create_user_record))
            )
            .service(resource("/{name}")
                .route(get().to(get_user_record_by_name))
                .route(post().to(add_user_record_by_name))
            )
        )
        // ------------ /user/{uid}/{rid} -------- /// 
        .service(scope("/{rid}")
            .service(resource("")
                .route(get().to(get_by_id))
                .route(put().to(update_user_record))
                .route(delete().to(delete_by_id))
            )
            // ------------ /user/{uid}/{rid}/items -------- /// 
            .service(scope("/items")
                .service(resource("")
                    .route(get().to(get_record_items))
                    .route(post().to(add_existing_item_to_record))
                )
                // ------------ /user/{uid}/{rid}/items/{name} -------- /// 
                .service(resource("/{name}")
                    .route(post().to(add_new_item_to_record_by_name)) 
                )
            )
            // ------------ /user/{uid}/{rid}/{iid} -------- /// 
            .service(scope("/{iid}")
                .service(resource("")
                    .route(post().to(get_record_item_by_id))
                )
            )
            // ------------ /user/{uid}/{rid}/rel -------- /// 
            .service(scope("/rel")
                .service(resource("")
                    .route(get().to(get_records_linked_with))
                )
                .service(resource("/{relation}")
                    .route(get().to(get_records_with_relation))
                    .route(post().to(add_record_with_relation))
                )
            )
        )
}


pub async fn get_user_records(
    data: web::Data<State>, uid: web::Path<i32>
) -> HttpResponse {
    match User::get_by_id(&data.db, *uid).await {
        Ok(Some(user)) => match User::get_all_records(&data.db, user.id.unwrap()).await {
            Ok(recs) => HttpResponse::Ok().json(recs),
            Err(_) => HttpResponse::NotFound().body(""),
        },
        _ => HttpResponse::NotFound().body(""),
    }
}

pub async fn get_by_id(id: web::Path<i32>, data: web::Data<State>) -> HttpResponse {
    match Record::get_by_id(&data.db, *id).await {
        Ok(rec) => HttpResponse::Ok().json(&rec), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}")
    }
}

pub async fn create_user_record(
    id: Identity,
    path: web::Path<i32>, 
    data: web::Data<State>,
    record: web::Json<Record>,) -> HttpResponse  //Should be RecordIn
{
    match record.into_inner().insert(&data.db).await {
        Ok(rec) => HttpResponse::Ok()
            .content_type("application/json")
            .json(&rec), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn delete_by_id(
    id: web::Path<i32>, data: web::Data<State>
) -> HttpResponse {
    match Record::delete_by_id(&data.db, *id).await {
        Ok(rec) => HttpResponse::Ok()
            .content_type("application/json")
            .json(&rec), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

/// TODO implement
pub async fn update_user_record(
    path: web::Path<i32>, data: web::Data<State>
) -> HttpResponse {
    match User::get_all_records(&data.db, *path).await {
        Ok(recs) => HttpResponse::Ok().json(&recs), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}")
    }
}

pub async fn add_new_item_to_record_by_name(
    path: web::Path<(i32, i32, String)>, data: web::Data<State>
) -> HttpResponse {
    let (_uid, rid, item_name) = path.into_inner().clone();
    match Record::add_new_item(&data.db, rid, item_name).await {
        Ok(item) => HttpResponse::Ok().json(&item), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}")
    }
}

/// TODO
pub async fn add_existing_item_to_record(
    path: web::Path<i32>, data: web::Data<State>
) -> HttpResponse {
        HttpResponse::NotFound().json("{}")
}

pub async fn get_records_linked_with(path: web::Path<i32>, data: web::Data<State>) -> HttpResponse {
    match User::get_linked_records(&data.db, *path).await {
        Ok(recs) => HttpResponse::Ok()
            .content_type("application/json")
            .json(&recs), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().finish()
    }
}

pub async fn get_records_with_relation(path: web::Path<i32>, data: web::Data<State>) -> HttpResponse {
    match User::get_linked_records(&data.db, *path).await {
        Ok(recs) => HttpResponse::Ok().json(&recs), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}")
    }

}

pub async fn get_user_record_by_name(
    id: Identity,
    path: web::Path<(i32, String)>, 
    data: web::Data<State>) -> HttpResponse 
{
    let (uid, rec_name) = path.into_inner();
    match User::get_named_record(&data.db, uid, rec_name).await {
        Ok(rec) => HttpResponse::Ok()
            .content_type("application/json")
            .json(&rec),
        Err(_) => HttpResponse::NotFound()
            .json("{}")
    }
}

pub async fn add_user_record_by_name(
    id: Identity,
    path: web::Path<(i32, String)>, 
    data: web::Data<State>) -> HttpResponse 
{
    let (uid, rec_name) = path.into_inner();
    match User::add_new_record(&data.db, uid, rec_name).await {
        Ok(rec) => HttpResponse::Created()
            .content_type("application/json")
            .json(&rec),
        Err(_) => HttpResponse::InternalServerError()
            .body("")
    }
}

pub async fn add_record_with_relation(path: web::Path<i32>, data: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json("{}")
}

pub async fn delete_record_by_uid_rid(path: web::Path<(i32, i32)>, data: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json("{}")
}

pub async fn delete_record_by_name(path: web::Path<(i32, String)>, data: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json("{}")
}
pub async fn get_record_items(path: web::Path<(i32, String)>, data: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json("{}")
}
pub async fn remove_item_from_record(path: web::Path<(i32, String)>, data: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json("{}")
}

pub async fn get_record_item_by_id(path: web::Path<(i32, String)>, data: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json("{}")
}

pub async fn get_record_by_id(path: web::Path<(i32, String)>, data: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json("{}")
}

pub async fn delete_record_by_id(path: web::Path<(i32, String)>, data: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json("{}")
}

