pub mod user;
pub mod record;

pub use user::User;
pub use record::Record;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::marker::{Send, Unpin};
use crate::db::Db;
use sqlx::{types::chrono::{Utc, DateTime}, FromRow, Type, postgres::{Postgres, PgRow}, Decode};


#[async_trait]
pub trait Model: Sized+ Default {
    //type Item;
    fn get_by_id(db: &Db, id: i32) -> sqlx::Result<()> { Ok(()) }
    fn by_id(id: i32) -> () {
        let query = sqlx::query
            ("SELECT * FROM $1 WHERE id=$2")
            .bind(Self::table())
            .bind(id);
    }
    fn create(db: &Db) -> sqlx::Result<u32> { Ok(0) }
    fn table() -> String;
    async fn get_all(db: &Db) -> sqlx::Result<Vec<Self>>;
    /*
    async fn get_all(&self, db: &Db) -> sqlx::Result<Vec<Self>>{
        let res: Vec<Self> = sqlx::query_as::<Postgres, Self>("SELECT * FROM ?")
            .bind(Self::table())
            .fetch_all(&db.pool).await?;
        Ok(())
    }
    */
}
