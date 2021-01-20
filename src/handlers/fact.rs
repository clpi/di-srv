use actix_multipart::Multipart;
use tokio::io::AsyncWriteExt;
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;
use crate::state::State;
use actix_web::{Scope,
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpResponse, HttpRequest
};
use div_db::models::{User, FactType, FactEntry};
use div_db::sqlx::{self, Postgres, query_as};

pub fn routes(base: &str) -> actix_web::Scope {
    scope(base)
        .service(resource("").route(get().to(get_all_types)))
        .service(resource("/entries").route(get().to(get_all_entries)))
        .service(scope("/{uid}")
            .service(resource("")
                .route(get().to(get_by_uid))
            )
        )
}

pub fn with_name() -> actix_web::Scope {
    scope("/{username}/{fact}")
}


pub async fn get_all_entries(
    id: actix_session::Session,
    data: web::Data<State>,) -> HttpResponse {
    let db = data.db.lock().unwrap();
    let res = sqlx::query_as::<Postgres, FactEntry>("SELECT * FROM FactEntries")
        .fetch_all(&db.pool).await.unwrap();
    HttpResponse::Ok()
        .json(&res)
}

pub async fn get_all_types(
    id: actix_session::Session,
    data: web::Data<State>,) -> HttpResponse {
    let db = data.db.lock().unwrap();
    let res = sqlx::query_as::<Postgres, FactType>("SELECT * FROM FactTypes")
        .fetch_all(&db.pool).await.unwrap();
    HttpResponse::Ok()
        .json(&res)
}

pub async fn get_by_uid(
    id: actix_session::Session,
    data: web::Data<State>,
    uid: web::Path<Uuid>,) -> HttpResponse {
    let db = data.db.lock().unwrap();
    let res = sqlx::query_as::<Postgres, FactType>("SELECT * FROM FactTypes WHERE uid = ?")
        .bind(&uid.to_string())
        .fetch_all(&db.pool).await.unwrap();
    HttpResponse::Ok()
        .json(&res)
}
