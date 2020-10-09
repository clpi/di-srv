use actix_web::{Error, web, HttpResponse, ResponseError};

#[derive(Debug)]
enum AuthError {
    Invalid,
    DoesNotExist,
    Internal,
}
