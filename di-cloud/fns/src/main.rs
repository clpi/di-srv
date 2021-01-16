use std::error::Error;
use lambda_runtime::{error::HandlerError, Context, lambda};
use serde::{Serialize, Deserialize};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    lambda!(handler);
    Ok(())
}

#[derive(Deserialize)]
pub struct Event {
    pub username: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct Output {
    out: String,
}

fn handler(e: Event, c: Context) -> Result<Output, HandlerError> {
    Ok(Output {out:  format!("Hello! {}", e.username) })
}
