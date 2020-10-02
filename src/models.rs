use serde::{Serialize, Deserialize};

pub struct Test {  }

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub res: bool,
}

impl Response {
    pub fn ok() -> Self { Self { res: true } }
    pub fn fail() -> Self { Self { res: false } }
}
