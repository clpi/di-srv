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
            .route(get().to(|| HttpResponse::Ok().body("")))
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
        .service(scope("/{iid}")
            .service(resource("")
                .route(get().to(|| HttpResponse::Ok().body("")))
                .route(put().to(|| HttpResponse::Ok().body("")))
                .route(delete().to(|| HttpResponse::Ok().body("")))
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

pub async fn get_user_items(
    id: Identity,
    uid: web::Path<i32>, 
    data: web::Data<State>) -> HttpResponse 
{
    println!("GET USER ITEMS: From {:?}", id.identity());
    match User::get_all_items(&data.db, *uid).await {
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
    match User::add_existing_item(&data.db, *uid, item.into_inner()).await {
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
    match User::add_new_item(&data.db, id, name).await {
        Ok(item) => HttpResponse::Ok()
            .content_type("application/json")
            .json(&item), //PgRow -> JSon?
        Err(_) => HttpResponse::NotFound().json("{}")
    }

}
