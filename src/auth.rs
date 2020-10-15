use actix_web::{
    web, 
};

pub struct CognitoRequest { 
    pub user: Option<String>,
    pub token: Option<String>,
}
pub struct AuthRequest {

}
