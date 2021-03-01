use actix_multipart::Multipart;
use tokio::io::AsyncWriteExt;
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;
use crate::state::State;
use actix_web::{Scope,
    get, post, put, delete,
    web::{self, scope, ServiceConfig},
    HttpResponse, HttpRequest,
};
use div_db::models::User;

pub fn routes(base: &str) -> actix_web::Scope {
    scope(base)
        .route("", web::get().to(get_all))
        .service(by_username())
        .service(by_uid())
}

pub async fn teste() ->HttpResponse {
    HttpResponse::Ok().body("/api/user/test")
}

pub fn by_username() -> actix_web::Scope {
    web::scope("/{username}")
        .route("", web::get().to(get_by_username))
        .route("", web::delete().to(delete_by_username))
        .route("", web::put().to(update_by_username))
}

pub fn by_uid() -> actix_web::Scope {
    web::scope("/{uid}")
        .route("", web::get().to(get_by_id))
        .route("", web::delete().to(delete_by_id))
        .route("", web::put().to(update_by_id))
}

pub async fn query_user(
    query: web::Query<UserQuery>,
    data: web::Data<State>,) -> actix_web::Result<HttpResponse>
{
    Ok(HttpResponse::Ok().body("D"))
}

pub async fn get_all(
    id: actix_session::Session,
    data: web::Data<State>,) -> actix_web::Result<HttpResponse>
{
    match User::get_all(&data.db.lock().unwrap()).await {
        Ok(users) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(&users)),
        Err(_) => Ok(HttpResponse::NotFound().json(""))
    }
}

pub async fn get_by_id(
    data: web::Data<State>,
    id: web::Path<String>) -> actix_web::Result<HttpResponse>
{
    let id: Uuid = Uuid::parse_str(id.into_inner().as_mut_str()).unwrap();
    match User::get_by_id(&data.db.lock().unwrap(), id).await {
        Ok(Some(user)) => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(&user)),
        _ => Ok(HttpResponse::NotFound().json(""))
    }
}

pub async fn update_by_id(
    path: web::Path<Uuid>,
    req: HttpRequest,
    data: web::Data<State>) -> actix_web::Result<HttpResponse>
{
    match User::delete_by_id(&data.db.lock().unwrap(), *path).await {
        Ok(Some(id)) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("Deleted user with id {}", id))),
        _ => Ok(HttpResponse::NotFound().body("Could not delete"))
    }
}

pub async fn delete_by_id(
    data: web::Data<State>,
    id: web::Path<Uuid>) -> actix_web::Result<HttpResponse>
{
    match User::delete_by_id(&data.db.lock().unwrap(), *id).await {
        Ok(Some(id)) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("Deleted user {:?}", id))),
        _ => Ok(HttpResponse::NotFound().json(""))
    }
}

pub async fn get_by_username(
    data: web::Data<State>,
    username: web::Path<String>) -> actix_web::Result<HttpResponse>
{
    let db = data.db.lock().unwrap();
    let u =  User::get_by_username(&db, username.into_inner());
    match u.await {
        Ok(Some(user)) => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(&user)),
        Ok(None) => Ok(HttpResponse::NotFound().json("Sorry")),
        _ => Ok(HttpResponse::NotFound().json("ERRROR")),
    }
}

pub async fn delete_by_username(
    id: actix_session::Session,
    data: web::Data<State>,
    username: web::Path<String>,) -> actix_web::Result<HttpResponse>
{
    let db = data.db.lock().unwrap();
    let u = User::delete_by_username(&db, username.to_string());
    match u.await {
        Ok(id) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("Deleted user {}", id))),
        Err(_) => Ok(HttpResponse::NotFound().json(""))
    }
}

pub async fn update_by_username(
    id: actix_session::Session,
    data: web::Data<State>,
    username: web::Path<String>) -> actix_web::Result<HttpResponse> {
    match User::delete_by_username(&data.db.lock().unwrap(), username.to_string()).await {
        Ok(id) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("Deleted user {}", id))),
        Err(_) => Ok(HttpResponse::NotFound().json(""))
    }
}


pub async fn get_user_info(data: web::Data<State>, rid: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}

pub async fn update_user_info(data: web::Data<State>, rid: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}

pub async fn get_user_feed(data: web::Data<State>, rid: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}

pub async fn get_uid_facts(data: web::Data<State>, rid: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}

pub async fn new_uid_fact(data: web::Data<State>, rid: web::Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().body("delete_record")
}

async fn upload_profile_picture(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filepath = format!("./tmp/{}", &filename);
        let mut f = tokio::fs::File::create(&filepath).await?;
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}


pub struct UserQuery {
    id: Option<Uuid>,
    u: Option<String>,
    e: Option<String>,
}
