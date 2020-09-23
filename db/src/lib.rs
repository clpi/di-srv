use sqlx::prelude::*;

#[cfg(feature="sqlite")]
use sqlx::{Sqlite, sqlite::*};

#[cfg(feature="pg")]
use sqlx::{Postgres, postgres::*};

#[cfg(feature="sqlite")]
pub async fn run_pg() -> () {}

#[cfg(feature="pg")]
pub async fn run_sqlite() -> () {}

pub struct Db {
}

impl Db {

    pub async fn init() -> () {}

}
