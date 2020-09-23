use actix_web::{
    get, HttpResponse, web, Responder
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route(web::resource("/login"), web::post().to(login))
            .route(web::resource("/signup"), web::post().to(signup))
            .service(web::get().to(get_all)));
}

#[get("/all")]
pub async fn get_all() -> HttpResponse {
    HttpResponse::Ok().body("all")
}

#[post("/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("all")
}

pub async fn signup() -> HttpResponse {
    HttpResponse::Ok().body("all")
}
