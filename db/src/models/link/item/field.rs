use serde::{Serialize, Deserialize};
use super::super::{Link, LinkedTo};
use crate::{ Db,
    models::{  Record, Item, Priority, Field, }, 
};
use sqlx::{FromRow, Postgres, types::chrono::{DateTime, Utc}, prelude::*};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct ItemFieldLink {
    #[serde(skip_serializing_if="Option::is_none")]
    id: Option<i32>,
    iid: i32,
    fid: i32,
    #[serde(default="Priority::default")]
    priority: Priority,
    #[serde(default="Utc::now")]
    created_at: DateTime<Utc>,
}

impl ItemFieldLink {

    pub async fn insert_new(self, db: &Db) -> sqlx::Result<u32> {
        let res = sqlx::query(
            "INSERT INTO ItemFieldLinks (iid, fid, created_at)
            VALUES ($1, $2, $3) RETURNING id")
            .bind(&self.iid)
            .bind(&self.fid)
            .bind(&self.created_at);
        let res = res.fetch_one(&db.pool).await?;
        Ok(res.get("id"))
    }

    pub async fn items_linked_to_field(self, db: &Db, fid: i32) -> sqlx::Result<Vec<Item>> {
        let res: Vec<Item> = sqlx::query_as::<Postgres, Item>(
           "SELECT i.id, i.name, i.uid, i.status, i.visibility, i.created_at
            FROM Items i INNER JOIN ItemFieldLinks if ON i.id=if.iid
            INNER JOIN Fields f ON if.fid=f.id AND f.id=$1")
            .bind(fid)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    pub async fn fields_linked_to_item(self, db: &Db, iid: i32) -> sqlx::Result<Vec<Field>> {
        let res: Vec<Field> = sqlx::query_as::<Postgres, Field>(
           "SELECT f.id, f.name, f.uid, f.field_type, f.field_value, 
                   f.created_at, f.visibility, f.created_at
            FROM Fields f INNER JOIN ItemFieldLinks if ON f.id=if.fid
            INNER JOIN Items i ON i.id=if.iid AND i.id=$1")
            .bind(iid)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }
}

impl Default for ItemFieldLink {
    fn default() -> Self {
        Self {
            id: None,
            iid: -1,
            fid: -1,
            priority: Priority::Unset,
            created_at: Utc::now(),
        }
    }
}

impl From<(Item, Field)> for ItemFieldLink {
    fn from((item, field): (Item, Field)) -> Self {
        Self { 
            iid: item.id.expect("Record has no id"),
            fid: field.id.expect("Item id not set"),
            ..Self::default()
        }
    }
}

impl From<(i32, i32)> for ItemFieldLink {
    fn from((iid, fid): (i32, i32)) -> Self {
        Self { iid, fid, ..Self::default() }
    }
}
