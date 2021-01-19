use div_api::app::create_app;
use actix_web::test::{self, TestRequest,};
use actix_web::{App, web, dev::Service, http};

pub fn user_service() -> () {

}

#[actix_rt::test]
async fn get_all_users_ok() -> actix_web::Result<()> {
    let mut app = test::init_service(create_app()).await;
    let req = test::TestRequest::get()
        .uri("/api/users")
        .to_request();
    let resp = app.call(req).await?;
    assert_eq!(resp.status(), http::StatusCode::OK);
    Ok(())
}
