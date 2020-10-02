use divapi::app;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7777").expect("Failed to bind");
    app::run_api(listener).await?;
    Ok(())
}
