use uuid::Uuid;
use actix_session::Session;
use crate::state::State;
use actix_web::{
    get, post, delete, put,
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse, Scope, Result,
};
use div_db::{
    models::{Item, Record, User},
    Db,
};

pub fn routes(base: &str) -> Scope {
    scope(base)
        .service(get_by_id)
        .service(delete_by_id)
        .service(update_by_id)
}

pub fn user_item_routes() -> Scope {
    scope("/user/{uid}/item")
        .service(
            resource("")
                .route(get().to(get_user_items))
                .route(put().to(add_item_to_user)),
        )
        .service(
            scope("/{name}")
                .service(
                    resource("")
                        .route(get().to(get_user_item))
                        .route(put().to(add_new_item_to_user))
                        .route(delete().to(delete_user_item)),
                )
                // ------------ /user/{uid}/item/{iid}/feed -------- ///
                .service(resource("/feed").route(get().to(|| HttpResponse::Ok().body(""))))
                // ------------ /user/{uid}/item/feed -------- ///
                .service(
                    scope("/feed")
                        .service(resource("").route(get().to(|| HttpResponse::Ok().body("")))),
                ),
        )
}

#[get("/{iid}")]
pub async fn get_by_id(
    id: Session,
    iid: web::Path<String>,
    data: web::Data<State>) -> actix_web::Result<HttpResponse>
{
    let id: Uuid = Uuid::parse_str(iid.into_inner().as_mut_str()).unwrap();
    match Item::get_by_id(&data.db.lock().unwrap(), id).await {
        Ok(rec) => Ok(HttpResponse::Ok().json("{}")), //PgRow -> JSon?
        Err(_) => Ok(HttpResponse::NotFound().json("{}")),
    }
}

#[delete("/{iid}")]
pub async fn delete_by_id(
    id: Session,
    iid: web::Path<String>,
    data: web::Data<State>,) -> actix_web::Result<HttpResponse>
{
    let iid: Uuid = Uuid::parse_str(iid.into_inner().as_mut_str()).unwrap();
    match Item::delete_by_id(&data.db.lock().unwrap(), iid).await {
        Ok(rec) => Ok(HttpResponse::Ok().json("{}")), //PgRow -> JSon?
        Err(_) => Ok(HttpResponse::NotFound().json("{}")),
    }
}

#[put("/{iid}")]
pub async fn update_by_id(
    id: Session,
    iid: web::Path<String>,
    data: web::Data<State>,
    item: web::Json<Item>,) -> actix_web::Result<HttpResponse> {
    let iid: Uuid = Uuid::parse_str(iid.into_inner().as_mut_str()).unwrap();
    match Item::update_by_id(&data.db.lock().unwrap(), iid, item.into_inner()).await {
        Ok(rec) => Ok(HttpResponse::Ok().json("{}")), //PgRow -> JSon?
        Err(_) => Ok(HttpResponse::NotFound().json("{}")),
    }
}

pub async fn get_user_item(
    id: Session, path: web::Path<(String, String)>, data: web::Data<State>,
) -> HttpResponse {
    let (mut uid, item_name) = path.into_inner();
    let uid = Uuid::parse_str(uid.as_mut_str()).unwrap();
    match User::get_item_by_name(&data.db.lock().unwrap(), uid, item_name).await {
        Ok(Some(item)) => HttpResponse::Ok().json(&item),
        _ => HttpResponse::NotFound().json("{}"),
    }
}

pub async fn delete_user_item(
    id: Session, path: web::Path<(String, String)>, data: web::Data<State>,
) -> HttpResponse {
    let (mut uid, item_name) = path.into_inner();
    let uid = Uuid::parse_str(uid.as_mut_str()).unwrap();
    match User::delete_item_by_name(&data.db.lock().unwrap(), uid, item_name).await {
        Ok(iid) => HttpResponse::Ok().json(&iid),
        _ => HttpResponse::NotFound().json("{}"),
    }
}

pub async fn get_user_items(
    id: Session, uid: web::Path<String>, data: web::Data<State>,
) -> HttpResponse {
    let uid = Uuid::parse_str(uid.into_inner().as_mut_str()).unwrap();
    match User::get_all_items(&data.db.lock().unwrap(), uid).await {
        Ok(rec) => HttpResponse::Ok().json("{}"), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}"),
    }
}

pub async fn add_item_to_user(
    id: Session, uid: web::Path<Uuid>, data: web::Data<State>, item: web::Json<Item>,
) -> HttpResponse {
    match User::add_existing_item(&data.db.lock().unwrap(), uid.into_inner(), item.into_inner()).await {
        Ok(item) => HttpResponse::Ok()
            .content_type("application/json")
            .json(&item), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}"),
    }
}

pub async fn add_new_item_to_user(
    id: Session,
    path: web::Path<(Uuid, String)>,
    data: web::Data<State>,
) -> HttpResponse {
    let (mut uid, name) = path.into_inner();
    match User::add_new_item(&data.db.lock().unwrap(), uid, name).await {
        Ok(item) => HttpResponse::Ok()
            .content_type("application/json")
            .json(&item), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}"),
    }
}
