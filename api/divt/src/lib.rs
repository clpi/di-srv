pub mod handlers;
pub mod middleware;
pub mod context;
pub mod routes;

pub use com::models::user::{User, UserLogin};
pub use tide::{
    http::Cookie,
    Response, StatusCode, Request
};


pub async fn run(host: &str, port: &str) -> tide::Result<()> {

    tide::log::start();

    let cx = context::create().await?;
    let mut app = tide::with_state(cx);

    app = middleware::set(app).await?;
    app = routes::set(app).await?;

    app.listen(format!("{}:{}", host, port)).await?;

    Ok(())
}

pub trait RequestExt {
    fn resp(&self) -> String;
}

impl<Context> RequestExt for tide::Request<Context> {
    fn resp(&self) -> String {
        "response".to_string()
    }
}



