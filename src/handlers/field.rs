use actix_web::{FromRequest, Scope,
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpResponse, HttpRequest
};
use uuid::Uuid;

pub fn routes() -> Scope {
    scope("/field")
    // -------------- /user ------------------------- ///
        .service(resource("").route(get().to(|| HttpResponse::Ok().body(""))))
        .service(scope("/{uid}")
        // -------------- /user/{uid} --------------------///
            .service(resource("")
                .route(get().to(get_by_id))
                .route(delete().to(delete_by_id))
            )
            // ------------ /user/{uid}/info/ -------- ///
            .service(resource("/info")
                .route(get().to(get_user_info))
                .route(put().to(update_user_info))
            )
        )
}

pub fn user_routes() -> Scope {
    scope("/{uid}/fields")
        .service(resource("").route(get().to(|| HttpResponse::Ok().body(""))))
}
