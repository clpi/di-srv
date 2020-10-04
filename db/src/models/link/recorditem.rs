use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, types::chrono::{DateTime, Utc}, 
    prelude::*, postgres::{PgRow, Postgres}
};
use crate::{ Db,
    models::{
        attrib::Attribute,
        Record, Item
    }, Db};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct RecordItemLink {
    #[serde(skip_serializing_if="Option::is_none")]
    id: Option<i32>,
    rid: i32,
    iid: i32,
    #[serde(default="Utc::now")]
    created_at: DateTime<Utc>,
}

impl RecordItemLink {

    pub fn new(record: Record, item: Item) -> Self {
        Self::from((record, item))
    }

    pub async fn insert(self, db: &Db) -> sqlx::Result<u32> {
        let res: u32 = sqlx::query
            ("INSERT INTO RecordItemLinks (rid, iid, created_at)
            VALUES ($1, $2, $3) RETURNING id")
            .bind(self.rid)
            .bind(&self.iid)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?
            .get("id");
        Ok(res)
    }

    pub async fn delete_by_id(self, db: &Db, id: i32) -> sqlx::Result<u32> {
        let res = sqlx::query
            ("DELETE FROM RecordItemLinks WHERE id=$1 RETURNING id")
            .bind(id)
            .fetch_one(&db.pool).await?
            .get("id");
        Ok(res)
    }

    pub fn add_attribute<T: Attribute>(self, attribute: T) -> () {}
}

impl Default for RecordItemLink {
    fn default() -> Self {
        Self {
            id: None,
            rid: -1,
            iid: -1,
            created_at: Utc::now(),
        }
    }
}

impl From<(Record, Item)> for RecordItemLink {
    fn from((record, item): (Record, Item)) -> Self {
        Self { 
            rid: record.id.expect("Record has no id"),
            iid: item.id.expect("Item id not set"),
            ..Self::default()
        }
    }
}

impl From<(i32, i32)> for RecordItemLink {
    fn from((rid, iid): (i32, i32)) -> Self {
        Self { rid, iid, ..Self::default() }
    }
}
