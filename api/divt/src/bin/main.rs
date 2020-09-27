#[async_std::main]
async fn main() -> async_std::io::Result<()> {
    divt::run("127.0.0.1", "3002").await.unwrap();
    Ok(())
}
