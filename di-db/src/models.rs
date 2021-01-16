pub mod user;
pub mod fact;
pub mod userinfo;
pub mod record;
pub mod item;
pub mod group;
pub mod relation;
pub mod link;

pub use user::User;
pub use userinfo::UserInfo;
pub use record::Record;
pub use item::Item;
pub use group::Group;
pub use link::Link;
pub use fact::{FactType, FactEntry};


pub use dynomite::{Attribute, Attributes, AttributeValue};

use async_trait::async_trait;
use sqlx::{ prelude::*,
    types::{
        chrono::{Utc, DateTime, NaiveDate, NaiveDateTime}, uuid::{Uuid, Variant},
    },
    FromRow, Type, postgres::{Postgres, PgRow}, Decode
};
use crate::Db;

#[async_trait]
pub trait Model: Sized + Default + Send + Sync {

    fn table() -> String;

    fn foreign_id() -> String;

    fn foreign_keys() -> Vec<String> { vec!["".to_string()] }

    fn id(self) -> Uuid;

    fn fields() -> Vec<String> { vec!["".to_string()] }

    fn values() -> Vec<String> { vec!["".to_string()] }

    async fn insert(&self, db: &Db) -> sqlx::Result<Uuid> { Ok(Uuid::new_v4()) }

    async fn delete(self, db: &Db) -> sqlx::Result<Uuid> {
        let res: Uuid = sqlx::query("DELETE FROM $1 WHERE id = $2 RETURNING id")
            .bind(Self::table())
            .bind(self.id())
            .fetch_one(&db.pool)
            .await?
            .get("id");
        Ok(res)
    }

    async fn fetch_from_id(db: &Db, id: Uuid) -> sqlx::Result<()> {
        let res: PgRow = sqlx::query("SELECT * FROM $1 WHERE id = $2")
            .bind(Self::table())
            .bind(id)
            .fetch_one(&db.pool)
            .await?;
        Ok(())
    }

    async fn delete_from_id(db: &Db, id: Uuid) -> sqlx::Result<Uuid> {
        let res: Uuid = sqlx::query_scalar
            ("DELETE FROM $1 WHERE id=$2 RETURNING id")
            .bind(Self::table())
            .bind(id)
            .fetch_one(&db.pool).await?;
        Ok(res)
    }

    async fn fetch_all(db: &Db, id: Uuid) -> sqlx::Result<Vec<PgRow>> {
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
