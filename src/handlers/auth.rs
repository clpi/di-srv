use crate::{state::State, models::UserIn};
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{
    http::{Cookie, HeaderName, HeaderValue},
    web::{self, delete, get, post, put, resource, scope, ServiceConfig},
    HttpRequest, HttpResponse,
};
use com::auth::Auth;
use divdb::models::user::*;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
    .service(scope("/auth")
        .service(resource("")
            .route(get().to(|| HttpResponse::Ok().body("GET /auth")))
            .route(get().to(|| HttpResponse::Ok().body("POST /auth")))
            .route(get().to(|| HttpResponse::Ok().body("DELETE /auth"))),
        )
        .service(resource("/login")
            .route(get().to(|| HttpResponse::Ok().body("GET /auth/login")))
            .route(post().to(login)),
        )
        .service(resource("/signup")
            .route(get().to(|| HttpResponse::Ok().body("GET /auth/signup")))
            .route(post().to(signup)),
        )
        .service(resource("/logout")
            .route(get().to(|| HttpResponse::Ok().body("GET /auth/logout")))
            .route(post().to(logout)),
        )
        .service(resource("/refresh")
            .route(get().to(check_id))
            .route(post().to(refresh_login)),
        ),
    );
}

pub async fn signup(
    (req, user, data): (HttpRequest, web::Json<User>, web::Data<State>),
) -> HttpResponse {
    //let user = user.clone();
    let hashed_user = User {
        password: Auth::new().hash(&user.password).unwrap(), ..user.clone()
    };
    println!("SIGNUP: {}", serde_json::to_string(&hashed_user).unwrap());
    match hashed_user.insert(&data.db).await {
        Ok(_uid) => {
            HttpResponse::Ok()
                .body("User signed up")
        },
        Err(_) => HttpResponse::NotAcceptable().finish()
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
            if Auth::new().verify(user.password, &db_user.password).unwrap() {
                let user_in = UserIn::from(db_user);
                let login_str = serde_json::to_string(&user_in).unwrap();
                id.remember(login_str.clone());
                HttpResponse::Ok()
                    .set_header("authorization", "true")
                    .content_type("application/json")
                    .json(&user_in)
            } else { HttpResponse::NotFound().body("Couldn't login") }
        }
        _ => HttpResponse::NotFound().body("COuldn't login")
    }
}

pub async fn logout(id: Identity) -> HttpResponse {
    match id.identity() {
        Some(_ident) => {
            id.forget();
            HttpResponse::Ok()
                .set_header("authorization", "false")
                .body("User logged out")
        }
        None => HttpResponse::NotFound().body("No user to log out"),
    }
}

pub async fn refresh_login(
    (id,  data): (Identity, web::Data<State>)) -> HttpResponse 
{
    match id.identity() {
        Some(id) => {
            println!("REFRESH: {}", id);
            let user: UserIn = serde_json::from_str(&id).unwrap();
            HttpResponse::Ok()
                .set_header("authorization", "true")
                .json(&user)
        },
        None => HttpResponse::Gone()
            .set_header("authorization", "false")
            .json(false)
        
    }
}

pub async fn check_id(
    (id, req, user, data): (
        Identity,
        HttpRequest,
        web::Json<UserLogin>,
        web::Data<State>,
    ),
) -> HttpResponse {
    match id.identity() {
        Some(id) => {
            let user: UserIn = serde_json::from_str(&id).unwrap();
            HttpResponse::Ok()
                .set_header("authorization", "true")
                .json(&user)
        }
        None => HttpResponse::NotFound()
            .set_header("authorization", "false")
            .json(false)
    }
}
