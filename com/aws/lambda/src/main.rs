use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Serialize, Deserialize};

fn main() {
    println!("Hello, world!");
}

#[derive(Deserialize)]
pub struct Event {
    username: String,
    email: String,
}

#[derive(Serialize)]
pub struct Output {

}

fn handler() -> Result<Output, HandlerError> {
    Ok(Output {  })
}
