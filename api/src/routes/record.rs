use actix_web::{
    get, HttpResponse,
};

#[get("/all")]
async fn get_all() -> HttpResponse {
    HttpResponse::Ok().body("all")
}
