use actix_web::web;
use crate::handlers::{user, auth};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/user")
        .service(
            web::resource("").route(
                web::get().to(user::get_all)
            )
        )
        .service(
            web::resource("/{id}").route(
                web::get().to(user::get_by_id)
            )
        )
    .service(
        web::scope("/auth")
        .service(
            web::resource("/login").route(
                web::post().to(auth::login)
            )
        )
        .service(
            web::resource("/signup").route(
                web::post().to(auth::signup)
            )
        )
        .service(
            web::resource("/logout").route(
                web::post().to(auth::logout)
            )
        )
    )
    );
}
