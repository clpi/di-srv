mod common;

use actix_web::test::{self, TestRequest};

#[actix_rt::test]
async fn post_record_works() {}

#[actix_rt::test]
async fn get_record_works() {}

#[actix_rt::test]
async fn delete_record_works() {}

#[actix_rt::test]
async fn get_nonexistent_record_fails() {}
