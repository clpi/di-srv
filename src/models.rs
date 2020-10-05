pub mod request;
pub mod response;

pub use request::*;
pub use response::*;

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
