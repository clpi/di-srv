use divapi::app;
// use env_logger;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // env_logger::from_env(Env::default().default_filter_or("info").init());
    let listener = TcpListener::bind("127.0.0.1:7777").expect("Failed to bind");
    app::run_api(listener).await?;
    Ok(())
}
