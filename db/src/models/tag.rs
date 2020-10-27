use sqlx::{FromRow,  Postgres,
    types::{
        chrono::{DateTime, Utc}, 
        uuid::{Uuid, Variant}
    }, 
    postgres::PgRow, prelude::*
};
use serde::{Serialize, Deserialize};
use crate::{Db, 
    models::{Model, User, Status, Visibility, Priority, Item, Group,
        link::{LinkedTo, Link},
    },
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Tag {
    #[serde(default="Uuid::new_v4")]
    pub id: Uuid,
    pub name: String,
    pub value: Option<String>,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct ItemTag {
    #[serde(default="Uuid::new_v4")]
    pub id: Uuid,
    pub tid: Uuid,
    pub iid: Uuid,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct RecordTag {
    #[serde(default="Uuid::new_v4")]
    pub id: Uuid,
    pub tid: Uuid,
    pub rid: Uuid,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct FieldTag{
    #[serde(default="Uuid::new_v4")]
    pub id: Uuid,
    pub tid: Uuid,
    pub fid: Uuid,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Default for Tag {
    fn default() -> Self {
        Tag {
            id: Uuid::new_v4(),
            name: String::new(),
            value: None,
            created_at: Utc::now(),
        }
    }
}

impl Default for FieldTag {
    fn default() -> Self {
        FieldTag { created_at: Utc::now(), ..Default::default() }
    }
}

impl Default for ItemTag {
    fn default() -> Self {
        ItemTag { created_at: Utc::now(), ..Default::default() }
    }
}

impl Default for RecordTag {
    fn default() -> Self {
        RecordTag { created_at: Utc::now(), ..Default::default() }
    }
}

impl Tag {

    pub async fn new<T, U>(db: &Db, name: T, value: Option<String>) -> sqlx::Result<Self> 
    where T: Into<String> {
        let tag_name: String = name.into();
        let res = sqlx::query_as::<Postgres, Tag>(
            "SELECT * FROM Tags WHERE name = $1 AND value = $2")
            .bind(tag_name.clone())
            .bind(&value)
            .fetch_optional(&db.pool)
            .await?;
        match res {
            Some(res) => { Ok(res) }
            None => {
                let res = sqlx::query_as::<Postgres, Tag>(
                    "INSERT INTO Tags (id, name, value, created_at)
                    VALUES ($1, $2, $3, $4) RETURNING *")
                    .bind(Uuid::new_v4())
                    .bind(tag_name)
                    .bind(&value)
                    .bind(Utc::now())
                    .fetch_one(&db.pool).await?;
                Ok(res)
            }
        }
    }

    pub async fn tag_record(self, db: Db, rid: Uuid) -> sqlx::Result<Uuid> {
        let res: Uuid = sqlx::query("INSERT INTO RecordTags (id, tid, rid, created_at)
            VALUES ($1, $2, $3, $4) RETURNING id")
            .bind(Uuid::new_v4())
            .bind(&self.id)
            .bind(rid)
            .bind(Utc::now())
            .fetch_one(&db.pool).await?
            .get("id");
        Ok(res)
    }

    pub async fn tag_item(self, db: Db, iid: Uuid) -> sqlx::Result<Uuid> {
        let res: Uuid = sqlx::query("INSERT INTO ItemTags (id, tid, iid, created_at)
            VALUES ($1, $2, $3, $4) RETURNING id")
            .bind(Uuid::new_v4())
            .bind(&self.id)
            .bind(iid)
            .bind(Utc::now())
            .fetch_one(&db.pool).await?
            .get("id");
        Ok(res)
    }

    pub async fn tag_field(self, db: Db, fid: Uuid) -> sqlx::Result<Uuid> {
        let res: Uuid = sqlx::query("INSERT INTO FieldTags (id, tid, fid, created_at)
            VALUES ($1, $2, $3, $4) RETURNING id")
            .bind(Uuid::new_v4())
            .bind(&self.id)
            .bind(fid)
            .bind(Utc::now())
            .fetch_one(&db.pool).await?
            .get("id");
        Ok(res)
    }
}

