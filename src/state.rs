use super::config::AppConfig;
use std::sync::{Arc, Mutex};
use div_cloud::cognito::{types::*, CognitoClient};
use actix::{Actor, Addr, Context, Handler};
use div_db::db::Db;

pub struct LoggedInUsers {}

#[derive(Clone)]
pub struct State {
    pub cognito: CognitoClient,
    pub db: Arc<Mutex<Db>>,
    pub tera: tera::Tera,
}

impl State {

    // TODO use config to determine params
    pub async fn new(_cf: &AppConfig) -> Self {
        let db = Db::new().await.expect("Could not get DB");
        let idp = CognitoClient::new();
        let mut tera = tera::Tera::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "assets/static/templates/**/*"))
            .expect("Could not load tera");
        tera.autoescape_on(vec!["html"]);
        Self { db: Arc::new(Mutex::new(db)), cognito: idp, tera }
    }

    pub fn new_blocking() -> Self {
        let db = Db::new_blocking().unwrap();
        let idp = CognitoClient::new();
        let _config = AppConfig::default();
        let mut tera = tera::Tera::new("assets/static/templates/**/*").expect("Could not load tera");
        tera.autoescape_on(vec!["html"]);
        Self { db: Arc::new(Mutex::new(db)), cognito: idp, tera }
    }
}

impl Actor for State {
    type Context = Context<Self>;
}
