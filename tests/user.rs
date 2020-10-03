mod common;

use actix_web::test::{self, TestRequest};

#[actix_rt::test]
async fn get_all_users_ok() {}

#[actix_rt::test]
async fn get_user_by_id_ok() {}

#[actix_rt::test]
async fn get_user_by_username_ok() {}
