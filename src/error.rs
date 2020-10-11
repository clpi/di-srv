use actix_web::{Error, web, HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug)]
enum AuthError {
    Invalid,
    DoesNotExist,
    Internal,
}
