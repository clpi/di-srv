use std::sync::{Arc, Mutex};
use div_cloud::cognito::{types::*, CognitoClient};
use actix::{Actor, Addr, Context, Handler};
use div_db::db::Db;

pub struct LoggedInUsers {}

pub struct DBConfig {
}

pub struct Config {
    db_url: String,
    api_key: Vec<u8>,
    secret_key: Vec<u8>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            db_url: String::new(),
            api_key: Vec::new(),
            secret_key: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct State {
    pub cognito: CognitoClient,
    pub db: Arc<Mutex<Db>>,
    pub tera: tera::Tera,
}

impl State {

    pub async fn new() -> Self {
        let db = Db::new().await.expect("Could not get DB");
        let idp = CognitoClient::new();
        let mut tera = tera::Tera::new("assets/static/templates/**/*").expect("Could not load tera");
        tera.autoescape_on(vec!["html"]);
        Self { db: Arc::new(Mutex::new(db)), cognito: idp, tera}
    }

    pub async fn new_blocking() -> Self {
        let db = Db::new_blocking().unwrap();
        let idp = CognitoClient::new();
        let mut tera = tera::Tera::new("assets/static/templates/**/*").expect("Could not load tera");
        tera.autoescape_on(vec!["html"]);
        Self { db: Arc::new(Mutex::new(db)), cognito: idp, tera}
    }
}

impl Actor for State {
    type Context = Context<Self>;
}
