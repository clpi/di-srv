pub mod user;
pub mod record;
pub mod item;
pub mod group;
pub mod userinfo;
pub mod link;
pub mod field;
pub mod relation;
pub mod entity;
pub mod logic;

pub use user::User;
pub use record::Record;
pub use item::Item;
pub use field::Field;
pub use link::*;
pub use entity::*;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::marker::{Send, Unpin};
use crate::{db::Db, models::link::Link};
use sqlx::{types::chrono::{Utc, DateTime}, FromRow, Type, postgres::{Postgres, PgRow}, Decode};

#[derive(sqlx::Type, Serialize, Deserialize, Clone)]
#[repr(i32)]
pub enum Status {
    Active = 1,
    Archived = 0,
    Deleted = -1,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone)]
#[repr(i32)]
pub enum Visibility {
    Private = 0,
    FriendsOnly = 1,
    Public = 2,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone)]
#[repr(i32)]
pub enum Priority {
    Lowest = 0,
    Low = 1,
    Medium = 3,
    High = 4,
    Highest = 5,
}

//pub trait Model<'r>: Sized + Default + FromRow<'r, PgRow> {
#[async_trait]
pub trait Model: Sized + Default {

    fn table() -> String;

    async fn get_by_id(db: &Db, id: i32) -> sqlx::Result<PgRow> {
        let res: PgRow = sqlx::query("SELECT * FROM $1 WHERE id = $2")
            .bind(Self::table())
            .bind(id)
            .fetch_one(&db.pool)
            .await?;
        Ok(res)
    }

    async fn delete_by_id(db: &Db, id: i32) -> sqlx::Result<u32> {
        let res: u32 = sqlx::query_scalar
            ("DELETE FROM $1 WHERE id=$2 RETURNING id")
            .bind(Self::table())
            .bind(id)
            .fetch_one(&db.pool).await?;
        Ok(res)
    }

    async fn get_all(db: &Db) -> sqlx::Result<Vec<PgRow>> {
        let res = sqlx::query("SELECT * FROM $1") 
            .bind(Self::table())
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }

}
