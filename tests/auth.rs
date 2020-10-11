mod common;

use actix_web::{App, web};
use actix_web::test;
use divapi::models::user::User;
use divdb::models::user::{UserRegister, UserLogin};
use divapi::{
    app::create_app, handlers::auth::{signup, login, logout, refresh_login},
};

/*
#[actix_rt::test]
async fn signup_works() {
    let mut app = test::init_service(create_app()).await;
    let user = UserRegister {
        username: "test123".into(), 
        password: "123test".into(), 
        email: "123test@test.com".into()
    };
    let req = test::TestRequest::post()
        .set_json(&user)
        .to_request();
    let res = test::call_service(&mut app, req).await;
    assert!(res.status().is_success());
    let get_user = User::get_by_username(&divapi::state::state().db.lock().unwrap(), "test123".into()).await.unwrap();
    assert!(get_user.is_some());
}

*/
#[actix_rt::test]
async fn login_works() {}

#[actix_rt::test]
async fn refresh_works() {}

#[actix_rt::test]
async fn logout_works() {}

#[actix_rt::test]
async fn identity_service_ok() {}

#[actix_rt::test]
async fn signup_existing_username_fails() {}

#[actix_rt::test]
async fn signup_blacklisted_username_fails() {}

#[actix_rt::test]
async fn login_username_not_found_fails() {}

#[actix_rt::test]
async fn login_wrong_password_fails() {}
