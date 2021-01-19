use div_api::app;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    app::run_api().await?;
    Ok(())
}
