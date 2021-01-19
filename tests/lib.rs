mod auth;
mod public;
mod session;

use div_api::app::create_app;
use actix_web::{
    body::MessageBody,
    middleware, test, App,
    web::{get, post, resource, Bytes},
};

pub async fn body<M>(route: &str) -> actix_web::Result<Bytes> {
    let srv = test::start(move || create_app());
    let mut resp = srv.get(route).send().await?;
    Ok(resp.body().await?)
}

