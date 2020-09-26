use sqlx::{
    prelude::*, Any, AnyPool,
};
use sqlx::{SqlitePool, sqlite::*};
use sqlx::{PgPool, postgres::PgPoolOptions}; 

#[cfg(feature="sqlite")]

#[cfg(feature="pg")]

#[cfg(feature="sqlite")]
pub async fn run_pg() -> () {}

#[cfg(feature="pg")]
pub async fn run_sqlite() -> () {}

pub async fn init() -> Result<(), sqlx::Error> {
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dotenv::var("DATABASE_URL").unwrap()).await?;
     
    Ok(())
}

pub struct Db {
}

impl Db {

    pub async fn init() -> () {}

}
