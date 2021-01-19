use div_api::app::create_app;
use actix_web::{
    middleware, test, App,
    web::{get, post, resource},
};

#[actix_rt::test]
async fn static_index_ok() -> actix_web::Result<()> {
    let srv = test::start(move || create_app());
    let mut resp = srv.get("/").send().await?;
    // let cookie = resp.cookies()?.clone()
    //     .into_iter()
    //     .find(|c| c.name() == "r-auth-cookie")?;
    Ok(())
}
