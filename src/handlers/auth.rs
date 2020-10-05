use crate::state::State;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{
    http::{Cookie, HeaderName, HeaderValue},
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};
use com::auth::Auth;
use divdb::models::user::*;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth")
            .service(
                resource("")
                    .route(get().to(|| HttpResponse::Ok().body("GET /auth")))
                    .route(get().to(|| HttpResponse::Ok().body("POST /auth")))
                    .route(get().to(|| HttpResponse::Ok().body("DELETE /auth"))),
            )
            .service(
                resource("/login")
                    .route(get().to(|| HttpResponse::Ok().body("GET /auth/login")))
                    .route(post().to(login)),
            )
            .service(
                resource("/signup")
                    .route(get().to(|| HttpResponse::Ok().body("GET /auth/signup")))
                    .route(post().to(signup)),
            )
            .service(
                resource("/logout")
                    .route(get().to(|| HttpResponse::Ok().body("GET /auth/logout")))
                    .route(post().to(logout)),
            )
            .service(
                resource("/refresh")
                    .route(get().to(|| HttpResponse::Ok().body("GET /auth/refresh")))
                    .route(post().to(refresh_login)),
            ),
    );
}

pub async fn signup(
    (req, user, data): (HttpRequest, web::Json<User>, web::Data<State>),
) -> HttpResponse {
    let user = user.clone();
    let hashed_user = User {
        password: Auth::new().hash(&user.password).unwrap(),
        ..user
    };
    match hashed_user.insert(&data.db).await {
        Ok(_uid) => {
            let mut resp = HttpResponse::Ok();
            resp.set_header("authorization", "true");
            resp.body("User signed up")
        }
        Err(_) => HttpResponse::NotAcceptable().body("Could not sign user up"),
    }
}

pub async fn login(
    (id, req, user, data): (
        Identity,
        HttpRequest,
        web::Json<UserLogin>,
        web::Data<State>,
    ),
) -> HttpResponse {
    let user = user.into_inner().clone();
    match User::get_by_username(&data.db, user.username).await {
        Ok(Some(db_user)) => {
            if Auth::new().verify(user.password, db_user.password).unwrap() {
                id.remember(db_user.username);
                let mut resp = HttpResponse::Ok();
                resp.set_header("authorization", "true")
                    .cookie(Cookie::new("authorized", "true"));
                resp.body("User {} signed in!")
            } else {
                HttpResponse::NotFound().body("User not signed in")
            }
        }
        _ => HttpResponse::NotFound().body("User not signed in"),
    }
}

pub async fn logout(
    (id, req, user, data): (
        Identity,
        HttpRequest,
        web::Json<UserLogin>,
        web::Data<State>,
    ),
) -> HttpResponse {
    match id.identity() {
        Some(_ident) => {
            id.forget();
            HttpResponse::Ok()
                .set_header("authorization", "false")
                .del_cookie(&Cookie::named("authorized"))
                .body("User logged out")
        }
        None => HttpResponse::NotFound().body("No user to log out"),
    }
}

pub async fn refresh_login(
    (id, req, user, data): (
        Identity,
        HttpRequest,
        web::Json<UserLogin>,
        web::Data<State>,
    ),
) -> HttpResponse {
    match id.identity() {
        Some(id) => match User::get_by_username(&data.db, id).await {
            Ok(Some(user)) => HttpResponse::Ok().json(true),
            _ => HttpResponse::Unauthorized()
                .set_header("authorization", "false")
                .del_cookie(&Cookie::named("authorized"))
                .json(true),
        },
        None => HttpResponse::Gone()
            .set_header("authorization", "false")
            .del_cookie(&Cookie::named("authorized"))
            .json(false)
        
    }
}
