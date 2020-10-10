use crate::{models::Response, state::State,};
use divdb::{Db, models::{Record, Item, User}};
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{ Scope,
    http::{Cookie, HeaderName, HeaderValue},
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};

pub fn base_routes() -> Scope {
    // ------------ /item -------- /// [ MAIN /src/handlers/items.rs ]
    scope("/item")
        .service(resource("")
            .route(get().to(|| HttpResponse::Ok().body("")))
        )
        .service(resource("{iid}")
            .route(get().to(get_by_id))
            .route(put().to(update_by_id))
            .route(delete().to(delete_by_id))
        )
}


pub fn user_item_routes() -> Scope {
    // ------------ /user/{uid}/item -------- /// [ MAIN /src/handlers/items.rs ]
    scope("/user/{uid}/item")
        .service(resource("")
            .route(get().to(get_user_items))
            .route(put().to(add_item_to_user))
        )
        // ------------ /user/{uid}/item/{iid} -------- ///
        .service(scope("/{name}")
            .service(resource("")
                .route(get().to(get_user_item))
                .route(put().to(add_new_item_to_user))
                .route(delete().to(delete_user_item))
            )
            // ------------ /user/{uid}/item/{iid}/feed -------- ///
            .service(resource("/feed")
                .route(get().to(|| HttpResponse::Ok().body("")))
        )
        // ------------ /user/{uid}/item/feed -------- ///
        .service(scope("/feed")
            .service(resource("")
                .route(get().to(|| HttpResponse::Ok().body("")))
            )
        )
    )
}

pub async fn get_by_id(id: Identity, iid: web::Path<i32>, data: web::Data<State>
    ) -> HttpResponse 
{
    match Item::get_by_id(&data.db.lock().unwrap(), *iid).await {
        Ok(rec) => HttpResponse::Ok().json("{}"), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}")
    }

}

pub async fn delete_by_id(id: Identity, iid: web::Path<i32>, data: web::Data<State>
    ) -> HttpResponse 
{
    match Item::delete_by_id(&&data.db.lock().unwrap(), *iid).await {
        Ok(rec) => HttpResponse::Ok().json("{}"), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}")
    }

}

pub async fn update_by_id(
    id: Identity, 
    iid: web::Path<i32>, 
    data: web::Data<State>,
    item: web::Json<Item>,
    ) -> HttpResponse 
{
    match Item::update_by_id(&data.db.lock().unwrap(), *iid, item.into_inner()).await {
        Ok(rec) => HttpResponse::Ok().json("{}"), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}")
    }

}

pub async fn get_user_item(
    id: Identity,
    path: web::Path<(i32, String)>, 
    data: web::Data<State>) -> HttpResponse 
{
    let (uid, item_name) = path.into_inner();
    match User::get_item_by_name(&data.db.lock().unwrap(), uid, item_name).await {
        Ok(Some(item)) => HttpResponse::Ok().json(&item),
        _ => HttpResponse::NotFound().json("{}")
    }

}

pub async fn delete_user_item(
    id: Identity,
    path: web::Path<(i32, String)>, 
    data: web::Data<State>) -> HttpResponse 
{
    let (uid, item_name) = path.into_inner();
    match User::delete_item_by_name(&data.db.lock().unwrap(), uid, item_name).await {
        Ok(iid) => HttpResponse::Ok().json(&iid),
        _ => HttpResponse::NotFound().json("{}")
    }

}

pub async fn get_user_items(
    id: Identity,
    uid: web::Path<i32>, 
    data: web::Data<State>) -> HttpResponse 
{
    println!("GET USER ITEMS: From {:?}", id.identity());
    match User::get_all_items(&data.db.lock().unwrap(), *uid).await {
        Ok(rec) => HttpResponse::Ok().json("{}"), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}")
    }

}

pub async fn add_item_to_user(
    id: Identity,
    uid: web::Path<i32>, 
    data: web::Data<State>,
    item: web::Json<Item>) -> HttpResponse 
{
    println!("ADD ITEM: From {:?}", id.identity());
    match User::add_existing_item(&data.db.lock().unwrap(), *uid, item.into_inner()).await {
        Ok(item) => HttpResponse::Ok()
            .content_type("application/json")
            .json(&item), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}")
    }

}

pub async fn add_new_item_to_user(
    id: Identity,
    path: web::Path<(i32, String)>, 
    data: web::Data<State>
) -> HttpResponse {
    println!("ADD NEW ITEM: From {:?}", id.identity());
    let (id, name) = path.into_inner();
    match User::add_new_item(&data.db.lock().unwrap(), id, name).await {
        Ok(item) => HttpResponse::Ok()
            .content_type("application/json")
            .json(&item), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}")
    }

}
