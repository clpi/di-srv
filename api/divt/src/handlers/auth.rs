pub use crate::{Request, Response, StatusCode, Cookie, UserLogin, User};
use com::auth::*;
use crate::context::Context;

//TODO implement hash verification
/*
pub async fn login(mut req: Request<Context>) -> tide::Result<Response> {
    let user: UserLogin = req.body_json().await.unwrap();
    let mut resp = Response::new(StatusCode::Ok);
    let pool = req.state().pool.clone();
    match User::from_username(pool, user.username).await {
        Ok(dbuser) =>
            if verify_pwd(&req.state().secret_key, &user.password, &dbuser.password).await {
                resp.insert_header("Login", "true");
                resp.set_body(format!("Signed in {}", dbuser.username));
                resp.insert_cookie(Cookie::new("auth", "token"));            
            } else {
                resp.insert_header("Login", "false");
                resp.set_body("Incorrect credentials");
            }
        Err(_) =>  {
            resp.insert_header("Login", "false"); 
            resp.set_body("Incorrect credentials");
        } 
    }
    Ok(resp)
}

pub async fn signup(mut req: Request<Context>) -> tide::Result<Response> {
    let mut user: User = req.body_json().await.unwrap();
    //user.password = hash_pwd(&req.state().secret_key, &user.password).await;
    let mut resp = Response::new(StatusCode::Ok);
    let pool = req.state().pool.clone();
    match user.insert(pool).await {
        Ok(_) => {
            let mut resp = Response::new(StatusCode::Ok);
            resp.insert_header("Register", "true"); //TODO actually implement real resp
            resp.set_body("Successfully signed up");
        },
        Err(_) =>  {
            let mut resp = Response::new(StatusCode::Unauthorized);
            resp.insert_header("Register", "false"); 
            resp.set_body("Could not sign up");
        },
    }
    Ok(resp)
}
*/
