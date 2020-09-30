use actix_web::{get, web, HttpResponse};

pub async fn signup() -> HttpResponse {
    HttpResponse::Ok().body("signup")
}

pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("login")
}

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("logout")
}
    
