use divapi::{app, context};
use std::net::tcplistener;

async fn main() -> std::io::result<()> {
    let listener = tcplistener::bind("127.0.0.1:7777").expect("failed to bind");
    app::run_api(listener).await?;
    ok(())
}
