use actix_web::{
    get, HttpResponse, web, Responder
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::post().to(signup));
    cfg.service(web::post().to(login));
    cfg.service(web::get().to(get_all));
}

//#[get("/all")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body("all")
}

//#[post("/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("all")
}

//#[post("/login")]
pub async fn signup() -> HttpResponse {
    HttpResponse::Ok().body("all")
}
