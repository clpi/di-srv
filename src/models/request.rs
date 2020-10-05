use actix_web::{self, web, HttpRequest, HttpResponse};

#[derive(FromRequest)]
pub struct ReqData<T> {
    path: web::Path<(String, String)>,
    query: web::Query<HashMap<String, String>>,
    json: web::Json<T>,
}
