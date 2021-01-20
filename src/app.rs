use super::config::AppConfig;
use std::collections::HashMap;
use crate::{handlers, middleware, state};
use actix_web_prom::PrometheusMetrics;
use actix_service::ServiceFactory;
use actix_web::{body, dev, get,  web, App, Error, HttpRequest, HttpResponse, HttpServer};
use std::{net::TcpListener, sync::mpsc};

pub async fn run_api() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    let config = AppConfig::default();
    let st = state::State::new(&config).await;
    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), "value1".to_string());
    let prometheus = PrometheusMetrics::new("", Some("/metrics"), Some(labels));
    let srv = HttpServer::new(move || {
        App::new()
            .data(st.clone())
            .wrap(middleware::cors())
            .wrap(middleware::logger())
            .wrap(prometheus.clone())
            .wrap(middleware::redis_session(&AppConfig::session_key()))
            .configure(handlers::public::routes)
            .configure(handlers::routes)
        });
    srv.bind(&config.clone().address())?
        .run().await?;
    Ok(())
}


pub fn spawn_api(
    listener: TcpListener,
    tx: mpsc::Sender<dev::Server>,
    ) -> std::io::Result<()>
{
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    log::info!("Spawning DI server...");

    let mut sys = actix_rt::System::new("api");
    let _db = sys.block_on(div_db::Db::new()).unwrap();
    let srv = HttpServer::new(move || create_app())
        .listen(listener)?
        .run();
    let _ = tx.send(srv.clone());
    sys.block_on(srv)
}


pub fn create_app() -> App<
    impl ServiceFactory<
        Config = (),
        Request = dev::ServiceRequest,
        Response = dev::ServiceResponse<body::Body>,
        Error = Error,
        InitError = (),
    >,
    body::Body,
> {
    let _config = AppConfig::default();
    let st = state::State::new_blocking();
    App::new()
        .data(st.clone())
        .wrap(middleware::cors())
        .wrap(middleware::redis_session(&AppConfig::session_key()))
        .configure(handlers::public::routes)
        .configure(handlers::routes)
}
