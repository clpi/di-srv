use div_api::app;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    app::run_api().await?;
    Ok(())
}
