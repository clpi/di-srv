pub use crate::{Request, Response, context::Context, StatusCode, Cookie, UserLogin, User};

pub async fn get_all(req: Request<Context>) -> tide::Result<Response> {
    match User::get_all(req.state().pool.clone()).await {
        Ok(users) => {
            let mut resp = Response::new(StatusCode::Ok);
            resp.set_body(serde_json::to_string(&users).unwrap());
            Ok(resp)
        },
        Err(_) => {
            let mut resp = Response::new(StatusCode::BadRequest);
            resp.set_body("Could not fetch users");
            Ok(resp)
        }
    }
}
