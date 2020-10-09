mod common;
use divapi::{
    models::{User, UserIn, UserQuery, UserRequest},
    handlers::user::{
        get_all, get_by_id, get_by_username, get_user_info, 
        update_by_id, update_user_info, update_by_username,
        delete_by_id, delete_by_username,
    },
};

use actix_web::test::{self, TestRequest};

pub async fn insert_test_user(num: i32, prefix: &str) -> () {  }

#[actix_rt::test]
async fn get_all_users_ok() {}

#[actix_rt::test]
async fn get_user_by_id_ok() {}

#[actix_rt::test]
async fn get_user_by_username_ok() {}


#[actix_rt::test]
async fn delete_user_by_id_ok() {

}
