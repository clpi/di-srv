use crate::context::Context;
use actix_web::{web::{self, resource, ServiceConfig, scope, put, get, delete, post}, HttpResponse, };
use divdb::models::{User, Record};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
    .service(scope("/user")
        .service(resource("")
            .route(get().to(get_all)))
        .service(scope("/id/{id}")
            .service(resource("")
                .route(get().to(get_by_id))
                .route(delete().to(delete_by_id)))
            .service(resource("/rec")
                .route(get().to(get_records)))
            .service(resource("/rec/{id}")
                .route(get().to(get_record))
                .route(put().to(update_record))
                .route(post().to(add_record))
                .route(delete().to(delete_record))))
        .service(resource("/{username}")
            .route(get().to(get_by_username))
            .route(put().to(update_by_username))
            .route(delete().to(delete_by_username))));
} 

///#[get("/user")]
pub async fn get_all(
    data: web::Data<Context>
) -> HttpResponse {
    let users: Vec<User> = User::get_all(&data.db).await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&users).unwrap())
}

///#[get("/user/id/{id}")]
pub async fn get_by_id(
    data: web::Data<Context>, id: web::Path<i32>
) -> HttpResponse {
    let user: User = User::get_by_id(&data.db, *id).await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&user).unwrap())
}

///#[delete("/user/id/{id}")]
pub async fn delete_by_id(
    data: web::Data<Context>, id: web::Path<i32>
) -> HttpResponse {
    let user: u32 = User::delete_by_id(&data.db, *id).await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("Deleted user {}", user))
}

///#[get("/user/{username}")]
pub async fn get_by_username(
    data: web::Data<Context>, username: web::Path<String>
) -> HttpResponse {
    let user: User = User::get_by_username(&data.db, username.to_string())
        .await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&user).unwrap())
}

///#[delete("/user/{username}")]
pub async fn delete_by_username(
    data: web::Data<Context>, username: web::Path<String>
) -> HttpResponse {
    let user: u32 = User::delete_by_username(&data.db, username.to_string())
        .await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("Deleted user {}", user))
}

///#[put("/user/{username}")]
pub async fn update_by_username(
    data: web::Data<Context>
) -> HttpResponse {
    HttpResponse::Ok().body("update_by_username")
}

///#[get("/user/id/{uid}/rec")]
pub async fn get_records(
    data: web::Data<Context>
) -> HttpResponse {
    HttpResponse::Ok().body("get_records")
}

///#[get("/user/id/{uid}/rec/{rid}")]
pub async fn get_record(
    data: web::Data<Context>, 
    path: web::Path<(i32, i32)>,
) -> HttpResponse {
    HttpResponse::Ok().body("get_record")
}

pub async fn add_record(
    data: web::Data<Context>, 
    uid: web::Path<i32>,
    record: web::Json<Record>,
) -> HttpResponse {
    HttpResponse::Ok().body("add_record")
}

pub async fn update_record(
    data: web::Data<Context>, rid: web::Path<i32>
) -> HttpResponse {
    HttpResponse::Ok().body("update_record")
}

pub async fn delete_record(
    data: web::Data<Context>, rid: web::Path<i32>
) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}

