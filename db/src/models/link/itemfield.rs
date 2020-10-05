use serde::{Serialize, Deserialize};
use crate::{ Db,
    models::{ attrib::Attrib, Record, Item, Priority, Field }, 
};
use sqlx::{FromRow, Postgres, types::chrono::{DateTime, Utc}};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct ItemFieldLink {
    #[serde(skip_serializing_if="Option::is_none")]
    id: Option<i32>,
    iid: i32,
    fid: i32,
    #[serde(default="Priority::unset")]
    priority: Priority,
    #[serde(default="Utc::now")]
    created_at: DateTime<Utc>,
}

impl ItemFieldLink {

    pub async fn insert(self, db: &Db) -> sqlx::Result<u32> {
        let res: u32 = sqlx::query
            ("INSERT INTO RecordItemLinks (rid, iid, created_at)
            VALUES ($1, $2, $3) RETURNING id")
            .bind(self.rid)
            .bind(&self.iid)
            .bind(&self.created_at);
        match res.fetch_one(&db.pool).await? {
            Ok(res) => Ok(res.get("id")),
            Err(_) => Err("Could not insert ItemFieldLink"),
        }
    }

    pub async fn items_linked_to_field(self, db: &Db, fid: i32) -> sqlx::Result<Vec<Item>> {
        let res: Vec<Item> = sqlx::query_as::<Postgres, Record>(
           "SELECT i.id, i.name, i.uid, i.status, i.visibility, i.created_at
            FROM Items i INNER JOIN ItemFieldLinks if ON i.id=if.iid
            INNER JOIN Fields f ON if.fid=f.id AND f.id=$1")
            .bind(fid);
        match res.fetch_all(&db.pool).await {
            Ok(res) => Ok(res),
            Err(_) => Err("Couldn't access ItemFieldLinks")
        }
    }

    pub async fn fields_linked_to_item(self, db: &Db, iid: i32) -> sqlx::Result<Vec<Field>> {
        let res: Vec<Field> = sqlx::query_as::<Postgres, Record>(
           "SELECT f.id, f.name, f.uid, f.field_type, f.field_value, 
                   f.created_at, f.visibility, f.created_at
            FROM Fields f INNER JOIN ItemFieldLinks if ON f.id=if.fid
            INNER JOIN Items i ON i.id=if.iid AND i.id=$1")
            .bind(iid);
        match res.fetch_all(&db.pool).await {
            Ok(res) => Ok(res),
            Err(_) => Err("Couldn't access ItemFieldLinks")
        }
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
