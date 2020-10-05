use crate::state::State;
use actix_web::{
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpResponse,
};
use divdb::models::{Record, User};

//TODO implement query string route handlers for user ids

pub fn routes(cfg: &mut ServiceConfig) {
    /// -------------- /user ------------------------- ///
    cfg.service(scope("/user")
        .service(resource("")
            .route(get().to(get_all))
        )
        /// -------------- /user/{uid} --------------------///
        .service(scope("/{uid}")
            .service(resource("")
                .route(get().to(get_by_id))
                .route(delete().to(delete_by_id))
            )
            /// ------------ /user/{uid}/info/ -------- ///
            .service(resource("/info")
                .route(get().to(get_user_info))
                .route(put().to(update_user_info))
            )
        )
        /// -------------- /user/{uid} --------------------///
    )
    /// -------------- /u ------------------------- ///
    .service(scope("/u")
        /// -------------- /u/{username} --------------------///
        .service(scope("/{username}")
            .service(resource("")
                .route(get().to(get_by_username))
                .route(put().to(update_by_username))
                .route(delete().to(delete_by_username)),
            )
            /// ------------ /u/{username}/feed/ -------- ///
            .service(scope("/feed")
                .service(resource("")
                    .route(get().to(get_user_feed))
                )
                /// ------------ /u/{username}/feed/items -------- ///
                .service(resource("/items")
                    .route(get().to(|| HttpResponse::Ok().body("")))
                )
                /// ------------ /u/{username}/feed/entities -------- ///
                .service(resource("/entities")
                    .route(get().to(|| HttpResponse::Ok().body("")))
                )
                /// ------------ /u/{username}/feed/records -------- ///
                .service(resource("/records")
                    .route(get().to(|| HttpResponse::Ok().body("")))
                )
            )
        ),
    );
}

pub async fn get_all(data: web::Data<State>) -> HttpResponse {
    let users: Vec<User> = User::get_all(&data.db).await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(&users)
}

pub async fn get_by_id(data: web::Data<State>, id: web::Path<i32>) -> HttpResponse {
    let user: User = User::get_by_id(&data.db, *id).await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(&user)
}

pub async fn delete_by_id(data: web::Data<State>, id: web::Path<i32>) -> HttpResponse {
    let user: u32 = User::delete_by_id(&data.db, *id).await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("Deleted user {}", user))
}

pub async fn get_by_username(data: web::Data<State>, username: web::Path<String>) -> HttpResponse {
    let user: User = User::get_by_username(&data.db, username.to_string())
        .await
        .unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(&user)
}

pub async fn delete_by_username(
    data: web::Data<State>,
    username: web::Path<String>,
) -> HttpResponse {
    let user: u32 = User::delete_by_username(&data.db, username.to_string())
        .await
        .unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("Deleted user {}", user))
}

pub async fn update_by_username(data: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().body("update_by_username")
}

pub async fn get_records(data: web::Data<State>, path: web::Path<i32>) -> HttpResponse {
    match User::get_by_id(&data.db, *path).await {
        Ok(user) => match user.get_all_records(&data.db).await {
            Ok(recs) => HttpResponse::Ok().json(recs),
            Err(_) => HttpResponse::NotFound().body(""),
        },
        Err(_) => HttpResponse::NotFound().body(""),
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
