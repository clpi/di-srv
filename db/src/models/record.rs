use std::rc::Weak;
use sqlx::{FromRow, types::chrono::{DateTime, Utc}, prelude::*, Postgres};
use serde::{Serialize, Deserialize};
use crate::{
    db::Db, models::{Model, link::{RecordItemLink, UserRecordLink}},
    models::{user::User, Status, Visibility, Priority,  item::Item,}
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Record {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub name: String,
    pub status: Status,
    pub visibility: Visibility,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

pub struct RecordObj {
    pub id: Option<i32>,
    pub user: Weak<User>,
    pub items: Vec<Weak<Item>>,
}

impl Record {

    pub fn new<T, U, V>(uid: i32, name: T, status: U, visibility: V) -> Self    
        where T: Into<String>,
              U: Into<Status>,
              V: Into<Visibility> {
        Self { 
            name: name.into(), 
            status: status.into(),
            visibility: visibility.into(),
            ..Self::default()
        }
    }

    pub fn name(self, name: String) -> Self {
        Self { name, ..self }
    }

    pub fn status(self, status: Status) -> Self {
        Self { status, ..self }
    }

    pub fn visibility(self, visibility: Visibility) -> Self {
        Self { visibility, ..self }
    }

    pub async fn get_by_id(db: &Db, id:  i32) -> sqlx::Result<Vec<Self>> {
        let res: Vec<Record> = sqlx::query_as::<Postgres, Record>(
            "SELECT * FROM Records WHERE id=$1")
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    pub async fn insert(self, db: &Db) -> sqlx::Result<u32> {
        let res: u32 = sqlx::query(
            "INSERT INTO Records (uid, name, status, visibility, created_at)
             VALUES ($1, $2, $3, $4, $5) RETURNING id")
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.status)
            .bind(&self.visibility)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?
            .get("id");
        Ok(res)
    }

    pub async fn add_item(self, db: &Db, item: Item) -> sqlx::Result<Self> {
        let link = RecordItemLink::from((self, item))
            .insert(&db).await?;
        Ok(self)
    }

    pub async fn delete_by_id(db: &Db, id:  i32) -> sqlx::Result<u32> {
        let res: u32 = sqlx::query_scalar(
            "DELETE FROM Records WHERE id=$1 RETURNING id")
            .fetch_one(&db.pool).await?;
        Ok(res)
    }

    pub async fn update_by_id(self, db: &Db, id: i32) -> sqlx::Result<u32> {
        Ok(0 as u32)
    }

}

impl Default for Record {
    fn default() -> Self {
        Self { 
            id: None, 
            uid: -1, 
            name: String::new(), 
            status: Status::Active.into(),
            visibility: Visibility::Private.into(),
            created_at: Utc::now(),
        } 
    }
}

impl From<Option<i32>> for Record {
    fn from(uid: Option<i32>) -> Self {
        Record { uid: uid.unwrap(), ..Record::default() }
    }
}

impl From<User> for Record {
    fn from(user: User) -> Self {
        Record { uid: user.id.unwrap(), ..Record::default() }
    }
}

#[async_trait::async_trait]
impl Model for Record {
    fn table() -> String { String::from("Records") }
}
