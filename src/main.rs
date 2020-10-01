use divapi::{app, context};
use std::net::TcpListener;

async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7777").expect("Failed to bind");
    app::run_api(listener).await?;
    Ok(())
}
