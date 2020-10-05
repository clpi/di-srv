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
use sqlx::{postgres::PgRow, prelude::*};
use crate::Db;

#[async_trait]
pub trait Model: Sized + Default {

    type ModelType;
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

    async fn is_in_db(self, db: &Db, model: Self) -> sqlx::Result<bool> {
        match self.id {
            Some(id) => match <Self as Model>::get_row_by_id(&db, id).await {
                Ok(row) => !row.is_empty(),
                Err(_)  => Err(sqlx::Error::Io()),
            },
            None => false,
        } 
    }

    async fn insert_db(self, db: &Db) -> sqlx::Result<i32>;

    async fn insert_if_not_in_db(mut self, db: &Db) -> sqlx::Result<Self> {
        if self.id == Some(-1) {
            match self.insert(db).await {
                Ok(id) => {
                    self.id = Some(id);
                    Ok(self)
                },
                Err(_) => Err(format!("Couldn't insert into {}", Self::table()).as_str())
            }
        } else { Ok(self) }
    }

}


