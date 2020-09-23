use actix_web::{
    get, HttpResponse,
};

#[get("/all")]
async fn login() -> HttpResponse {
    HttpResponse::Ok().body("all")
}

