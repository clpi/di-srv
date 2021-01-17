use actix_multipart::Multipart;
use tokio::io::AsyncWriteExt;
use futures::{StreamExt, TryStreamExt};
use uuid::Uuid;
use crate::state::State;
use actix_web::{Scope,
    web::{self, delete, get, post, put, resource, scope},
    HttpResponse, HttpRequest
};
use div_db::models::User;
use serde::{Serialize, Deserialize};

pub fn uid_routes() -> Scope {
    scope("/fact")
    // -------------- /user ------------------------- ///
        .service(resource("").route(get().to(get_all)))
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
