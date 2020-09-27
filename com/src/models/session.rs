use sqlx::{types::chrono::{Utc, DateTime}, FromRow, Type};
use serde::{Serialize, Deserialize};

#[derive(Clone, FromRow, Debug)]
pub struct Session {
  pub key: String,
  pub csrf: String,
  pub account: i32,
  pub identity: i32,
  pub expiry: DateTime<Utc>,
  pub invalidated: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: Option<DateTime<Utc>>,
}
