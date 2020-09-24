use actix_web::{
    get, HttpResponse, web
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::route(web::resource("/login"), web::post().to(login))
        .route(web::resource("/signup"), web::post().to(signup))
        .route(web::resource("/db_login"), web::get().to(get_all))
    );
}

//#[get("/all")]
pub fn get_all() -> HttpResponse {
    HttpResponse::Ok().body("all")
}

//#[post("/login")]
pub fn login() -> HttpResponse {
    HttpResponse::Ok().body("all")
}

//#[get("/all")]
pub fn signup() -> HttpResponse {
    HttpResponse::Ok().body("all")
}
