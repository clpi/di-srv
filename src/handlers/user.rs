use crate::{state::State, models::request::AuthRequest};
use actix_web::{FromRequest,
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpResponse,
};
use divdb::models::{Record, User, UserInfo,};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserApi(User);

//TODO implement query string route handlers for user ids

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("/user")
    // -------------- /user ------------------------- ///
        .service(resource("").route(get().to(get_all)))
        .service(scope("/{uid}")
        // -------------- /user/{uid} --------------------///
            .service(resource("")
                .route(get().to(get_by_id))
                .route(delete().to(delete_by_id))
            )
            // ------------ /user/{uid}/info/ -------- ///
            .service(resource("/info")
                .route(get().to(get_user_info))
                .route(put().to(update_user_info))
            )
        )
        // -------------- /user/{uid} --------------------///
    )
    // -------------- /u ------------------------- ///
    .service(scope("/u")
        .service(resource("").route(get().to(|| HttpResponse::Ok().body("/u"))))
        // -------------- /u/{username} --------------------///
        .service(scope("/{username}")
            .service(resource("")
                .route(get().to(get_by_username))
                .route(put().to(update_by_username))
                .route(delete().to(delete_by_username)),
            )
            // ------------ /u/{username}/feed/ -------- ///
            .service(scope("/feed")
                .service(resource("")
                    .route(get().to(get_user_feed))
                )
                // ------------ /u/{username}/feed/items -------- ///
                .service(resource("/items")
                    .route(get().to(|| HttpResponse::Ok().body("")))
                )
                // ------------ /u/{username}/feed/entities -------- ///
                .service(resource("/entities")
                    .route(get().to(|| HttpResponse::Ok().body("")))
                )
                // ------------ /u/{username}/feed/records -------- ///
                .service(resource("/records")
                    .route(get().to(|| HttpResponse::Ok().body("")))
                )
            )
        ),
    );
}
//TODO programmatically handle requests by matching operation to user model function

pub async fn get_all(data: web::Data<State>) -> HttpResponse {
    match User::get_all(&data.db).await {
        Ok(users) => HttpResponse::Ok()
            .content_type("application/json")
            .json(&users),
        Err(_) => HttpResponse::NotFound().json("")
    }
}

pub async fn get_by_id(data: web::Data<State>, id: web::Path<i32>) -> HttpResponse {
    match User::get_by_id(&data.db, *id).await {
        Ok(maybe_user) => match maybe_user {
            Some(user) => HttpResponse::Ok()
                .content_type("application/json")
                .json(&user),
            None => HttpResponse::NotFound().json(""),
        },
        Err(_) => HttpResponse::NotFound().json(""),
    }
}

pub async fn delete_by_id(
    data: web::Data<State>, 
    id: web::Path<i32>
) -> HttpResponse {
    match User::delete_by_id(&data.db, *id).await {
        Ok(id) => HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("Deleted user {}", id)),
        Err(_) => HttpResponse::NotFound().json("")
    }
}

pub async fn get_by_username(
    data: web::Data<State>, 
    username: web::Path<String>
) -> HttpResponse {
    match User::get_by_username(&data.db, username.to_string()).await {
        Ok(Some(user)) => HttpResponse::Ok()
                .content_type("application/json")
                .json(&user),
        _ => HttpResponse::NotFound().json(""),
    }
}

pub async fn delete_by_username(
    data: web::Data<State>,
    username: web::Path<String>,
) -> HttpResponse {
    match User::delete_by_username(&data.db, username.to_string()).await {
        Ok(id) => HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("Deleted user {}", id)),
        Err(_) => HttpResponse::NotFound().json("")
    }
}

pub async fn update_by_username(
    data: web::Data<State>, 
    username: web::Path<String>
) -> HttpResponse {
    match User::delete_by_username(&data.db, username.to_string()).await {
        Ok(id) => HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("Deleted user {}", id)),
        Err(_) => HttpResponse::NotFound().json("")
    }
}

pub async fn get_records(
    data: web::Data<State>, 
    path: web::Path<i32>
) -> HttpResponse {
    match User::get_by_id(&data.db, *path).await {
        Ok(Some(user)) => match user.get_all_records(&data.db).await {
            Ok(recs) => HttpResponse::Ok().json(recs),
            Err(_) => HttpResponse::NotFound().body(""),
        },
        _ => HttpResponse::NotFound().body(""),
    }
}

pub async fn get_user_info(data: web::Data<State>, rid: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}

pub async fn update_user_info(data: web::Data<State>, rid: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}

pub async fn get_user_feed(data: web::Data<State>, rid: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}
