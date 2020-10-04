pub mod user;
pub mod record;
pub mod item;
pub mod group;
pub mod userinfo;
pub mod link;
pub mod field;

pub use user::User;
pub use userinfo::UserInfo;
pub use record::Record;
pub use item::Item;
pub use field::Field;
pub use group::Group;
pub use types::{Visibility, Status, Priority};
pub use attrib::Attribute;

pub mod relation;
pub mod entity;
pub mod logic;
pub mod types;
pub mod attrib;

use async_trait::async_trait;
use sqlx::{postgres::PgRow, prelude::*};
use crate::Db;

#[async_trait]
pub trait Model: Sized + Default {

    fn table() -> String;

    async fn get_row_by_id(db: &Db, id: i32) -> sqlx::Result<PgRow> {
        let res: PgRow = sqlx::query("SELECT * FROM $1 WHERE id = $2")
            .bind(Self::table())
            .bind(id)
            .fetch_one(&db.pool)
            .await?;
        Ok(res)
    }

    async fn delete_row_by_id(db: &Db, id: i32) -> sqlx::Result<u32> {
        let res: u32 = sqlx::query_scalar
            ("DELETE FROM $1 WHERE id=$2 RETURNING id")
            .bind(Self::table())
            .bind(id)
            .fetch_one(&db.pool).await?;
        Ok(res)
    }

    async fn get_all_rows(db: &Db) -> sqlx::Result<Vec<PgRow>> {
        let res = sqlx::query("SELECT * FROM $1") 
            .bind(Self::table())
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }

}


