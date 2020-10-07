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

pub mod relation;
pub mod entity;
pub mod logic;
pub mod types;
pub mod attrib;

pub use types::{Visibility, Status, Priority};
pub use attrib::Attribute;

use async_trait::async_trait;
use sqlx::{postgres::PgRow, prelude::*, Postgres, Encode};
use crate::Db;

#[async_trait]
pub trait Model: Sized + Default + From<&'static PgRow> + Send + Sync {

    fn table() -> String;

    fn foreign_id() -> String;

    fn foreign_keys() -> Vec<String> { vec!["".to_string()] }

    fn id(self) -> i32;

    fn fields() -> Vec<String> { vec!["".to_string()] }

    fn values() -> Vec<String> { vec!["".to_string()] }

    async fn insert(&self, db: &Db) -> sqlx::Result<i32> { Ok(0i32) }

    async fn delete(self, db: &Db) -> sqlx::Result<i32> {
        let res: i32 = sqlx::query("DELETE FROM $1 WHERE id = $2 RETURNING id")
            .bind(Self::table())
            .bind(self.id())
            .fetch_one(&db.pool)
            .await?
            .get("id");
        Ok(res)
    }

    async fn fetch_from_id(db: &Db, id: i32) -> sqlx::Result<()> {
        let res: PgRow = sqlx::query("SELECT * FROM $1 WHERE id = $2")
            .bind(Self::table())
            .bind(id)
            .fetch_one(&db.pool)
            .await?;
        Ok(())
    }

    async fn delete_from_id(db: &Db, id: i32) -> sqlx::Result<i32> {
        let res: i32 = sqlx::query_scalar
            ("DELETE FROM $1 WHERE id=$2 RETURNING id")
            .bind(Self::table())
            .bind(id)
            .fetch_one(&db.pool).await?;
        Ok(res)
    }

    async fn fetch_all(db: &Db, id: i32) -> sqlx::Result<Vec<PgRow>> {
        let res: Vec<PgRow> = sqlx::query("SELECT * FROM $1 where id=$1") 
            .bind(Self::table())
            .bind(id)
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }
}

pub struct ModelQuery<'r, T: Model> {
    pub model: &'r T,
}
