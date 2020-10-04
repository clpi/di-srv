use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}, prelude::*};
use crate::{models::{Record, Item}, Db};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct RecordItemLink {
    id: Option<i32>,
    rid: i32,
    iid: i32,
    created_at: DateTime<Utc>,
}

impl RecordItemLink {

    pub fn new(record: Record, item: Item) -> Self {
        Self::from((record, item))
    }

    /*
    pub async fn insert(self, db: &Db) -> sqlx::Result<u32> {
        let res = sqlx::query!("INSERT INTO RecordItemLinks (rid, iid, created_at)
            VALUES ($1, $2, $3)", &self.rid, &self.iid, &self.created_at)
            .execute(&db.pool).await?;
        Ok(res.rows_affected() as u32)
    }

    pub async fn delete_by_id(self, db: &Db, id: i32) -> sqlx::Result<u32> {
        let res = sqlx::query!("INSERT INTO RecordItemLinks (rid, iid, created_at)
            VALUES ($1, $2, $3)", &self.rid, &self.iid, &self.created_at)
            .execute(&db.pool).await?;
        Ok(res.rows_affected() as u32)
    }
    */
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
