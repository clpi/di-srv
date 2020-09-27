pub mod routes;
pub mod auth;
pub mod context;
pub mod app;
pub mod types;
pub mod handlers;
pub mod errors;

use app::Api;
use std::net::TcpListener;
//use env_logger::Env;

pub async fn run_dev() -> () {

}

pub async fn run_prod() -> () {

}
