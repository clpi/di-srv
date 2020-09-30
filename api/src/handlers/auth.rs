use crate::context::Context;
use com::auth::Auth;
use divdb::models::user::*;
use actix_web::{web::{self, resource, ServiceConfig, scope, put, get, delete, post}, HttpResponse, HttpRequest, http::{HeaderValue, HeaderName, Cookie}};
use actix_identity::{Identity, IdentityService, CookieIdentityPolicy};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
    .service(scope("/auth")
        .service(resource("")
            .route(get().to(|| HttpResponse::Ok().body("GET /auth")))
            .route(get().to(|| HttpResponse::Ok().body("POST /auth")))
            .route(get().to(|| HttpResponse::Ok().body("DELETE /auth"))))
        .service(resource("/login")
            .route(get().to(|| HttpResponse::Ok().body("GET /auth/login")))
            .route(post().to(login)))
        .service(resource("/signup")
            .route(get().to(|| HttpResponse::Ok().body("GET /auth/signup")))
            .route(post().to(signup)))
        .service(resource("/logout")
            .route(get().to(|| HttpResponse::Ok().body("GET /auth/logout")))
            .route(post().to(logout)))
        .service(resource("/refresh")
            .route(get().to(|| HttpResponse::Ok().body("GET /auth/refresh")))
            .route(post().to(refresh_login)))
    );
} 

pub async fn signup(
    (req, user, data): (HttpRequest, web::Json<User>, web::Data<Context>)
) -> HttpResponse {
    let user = user.clone();
    let hashed_user = User {
        password: Auth::new().hash(&user.password).unwrap(), ..user 
    };
    let db = data.db.clone();
    let mut resp = match hashed_user.insert(&db).await {
        Ok(uid) => { HttpResponse::Ok() },
        Err(_) => { HttpResponse::NotAcceptable() },
    };
    resp.set_header("authorization", "true");
    resp.finish()
}

pub async fn login(
    (req, user, data): (HttpRequest, web::Json<UserLogin>, web::Data<Context>)
) -> HttpResponse {
    let mut resp = HttpResponse::Ok().body("login");
    let user = user.into_inner().clone();
    let db = data.db.clone();
    let mut resp = match User::get_by_username(&db, user.username).await {
        Ok(db_user) => { 
            if Auth::new().verify(user.password, db_user.password).unwrap() {
                HttpResponse::Ok()
            } else {
                HttpResponse::NotFound()
            }
        },
        Err(_) => HttpResponse::NotFound(), //might be insecure, too specific
    };
    resp.set_header("authorization", "true") 
        .cookie(Cookie::new("authorized", "true")); //I know this isn't how you do this
    resp.finish()
}

pub async fn logout(
    (req, user, data): (HttpRequest, web::Json<UserLogin>, web::Data<Context>)
) -> HttpResponse {
    HttpResponse::Ok().body("logout")
}

pub async fn refresh_login(
    (req, user, data): (HttpRequest, web::Json<UserLogin>, web::Data<Context>)
) -> HttpResponse {
    HttpResponse::Ok().body("refresh_login")
}
