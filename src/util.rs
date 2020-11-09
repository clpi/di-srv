use crate::state::State;
use actix_web::client::Client;


pub async fn req_get(url: &str) {
    let mut client = Client::default();
    let mut res = client.get(url).send().await.unwrap();
}

#[macro_export]
macro_rules! def_route {
    ($($name:ident, $handler:ident, $path:literal),*) => {
        $(
            pub async fn $name(
                data: Data<Context>,
                path: Path<String>,
                req: HttpRequest
            ) -> HttpResponse {
                HttpResponse::Ok().body("")
            }
        )*
        struct name {
            field: u32,
        }
    }
}

#[macro_export]
macro_rules! register_route {
    ($name:ident) => {};
}
