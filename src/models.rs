pub mod request;
pub mod response;
pub mod user;
pub mod auth;

pub use request::*;
pub use response::*;
pub use user::*;
pub use auth::*;

use serde::{Deserialize, Serialize};

pub struct Test {}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub res: bool,
}

impl Response {
    pub fn ok() -> Self {
        Self { res: true }
    }
    pub fn fail() -> Self {
        Self { res: false }
    }
}


