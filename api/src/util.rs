use actix_web::{web::{Data, Path}, http, HttpResponse, HttpRequest};
use crate::context::Context;

#[macro_export]
macro_rules! def_route {
    ($($name:ident, $handler:ident, $path:literal),*) => {
        $(
            pub async fn $name(
                data: Data<Context>, 
                path: Path<String>, 
                req: HttpRequest
            ) -> HttpResponse {
                HttpResponse::Ok().body("")    
            }
        )*
        struct name {
            field: u32,
        }
    }
}

#[macro_export]
macro_rules! register_route {
    ($name:ident) => {}
}
