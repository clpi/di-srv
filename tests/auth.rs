mod common;

use actix_web::test;

#[actix_rt::test]
async fn signup_works() {}

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
