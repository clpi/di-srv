use std::sync::{Arc, Mutex};
use div_cloud::cognito::{types::*, CognitoClient};
use actix::{Actor, Addr, Context, Handler};
use std::collections::HashMap;
use divdb::db::Db;
use actix_web::{self, web, HttpRequest, HttpResponse};
use actix_session::{Session, UserSession};
use actix_redis::{RedisActor, RedisSession};
use actix_web::{Either, client::Client };

pub fn state() -> State {
    let db = Db::new_blocking().unwrap();
    let state = State { db: Arc::new(Mutex::new(db)), cognito: CognitoClient::new() };
    state
}

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
}

impl State {

    pub async fn new() -> Self {
        let db = Db::new().await.expect("Could not get DB");
        let idp = CognitoClient::new();
        Self { db: Arc::new(Mutex::new(db)), cognito: idp, }
    }
}

impl Actor for State {
    type Context = Context<Self>;
}
