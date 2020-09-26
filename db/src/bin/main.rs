use serde::{Serialize, Deserialize};
use lib::db::*;
use sqlx::{
    prelude::*, Any, AnyPool,
};
use sqlx::{SqlitePool, sqlite::*};
use sqlx::{PgPool, postgres::PgPoolOptions}; 

#[cfg(feature="sqlite")]
pub mod sqlite {
    use super::*;
    use sqlx::{SqlitePool, sqlite::*};
    pub async fn run_sqlite() -> () {}
}


#[cfg(feature="pg")]
pub mod pg {
    use super::*;
    use sqlx::{PgPool, postgres::PgPoolOptions}; 
    pub async fn run_pg() -> () {}
}

#[async_std::main]
pub async fn main() -> sqlx::Result<()> {
    lib::db::init().await?;
    Ok(())

}
