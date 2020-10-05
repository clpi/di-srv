use std::rc::Weak;
use sqlx::{FromRow, types::chrono::{DateTime, Utc}, prelude::*, Postgres};
use serde::{Serialize, Deserialize};
use crate::{Db, 
    models::{Model, User, Status, Visibility, Priority, Item,
        link::{LinkModel, Link, recorditem::RecordItemLink, UserRecordLink}
    },
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Record {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub name: String,
    #[serde(default="Status::default")]
    pub status: Status,
    #[serde(default="Visibility::default")]
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

    pub fn new<T, U>(uid: T, name: U) -> Self 
    where T: Into<i32>, U: Into<String> {
        Self { 
            name: name.into(),  
            uid: uid.into(),
            ..Self::default()
        }
    }

    pub fn create<T, U, V, W>
        (uid: T, name: U, status: V, visibility: W) -> Self    
        where T: Into<i32>, U: Into<String>, 
              V: Into<Status>, W: Into<Visibility> {
        Self { 
            name: name.into(),
            uid: uid.into(),
            status: status.into(),
            visibility: visibility.into(),
            ..Self::default()
        }
    }

    fn build() -> () {}


    pub fn name<T>(self, name: T) -> Self where T: Into<String> {
        Self { name: name.into(), ..self }
    }

    pub fn status<T>(self, status: T) -> Self where T: Into<Status> {
        Self { status: status.into(), ..self }
    }

    pub fn visibility<T>(self, visibility: T) -> Self where T: Into<Visibility>{
        Self { visibility: visibility.into(), ..self }
    }

    pub async fn get_by_id(db: &Db, id: i32) -> sqlx::Result<Vec<Self>> {
        let res: Vec<Record> = sqlx::query_as::<Postgres, Record>(
            "SELECT * FROM Records WHERE id=$1")
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    pub async fn insert(mut self, db: &Db) -> sqlx::Result<Self> {
        let res = sqlx::query(
            "INSERT INTO Records (uid, name, status, visibility, created_at)
             VALUES ($1, $2, $3, $4, $5) RETURNING id")
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.status)
            .bind(&self.visibility)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?;
        let id: i32 = res.get("id");
        self.id = Some(id);
        let link = UserRecordLink::from(
            (self.uid, self.id.expect("No record ID set")));
        link
            .insert(&db)
            .await?;
        Ok(self)
    }

    pub async fn get_linked_users(self, db: &Db) -> sqlx::Result<Vec<User>> {
        let res = sqlx::query_as::<Postgres, User>
            ("SELECT u.id, u.username, u.email, u.created_at
              FROM Users u INNER JOIN UserRecordLinks ur ON u.id = ur.uid
                   INNER JOIN Records r on ur.rid = r.id
                   AND r.id = $1")
            .bind(&self.id.expect("No Record ID set"))
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }

    // Create new item, insert it into DB, and insert RecItemLink into DB
    // for *ALREADY* in-DB Record
    pub async fn add_new_item<T: Into<String>>
        (self, db: &Db, item_name: T) -> sqlx::Result<Self> 
    {
        let rec = match self.id {
            Some(_id) => self.clone(),
            None => self.insert(db).await?,
        };
        let item = Item::new(rec.id.unwrap(), item_name.into())
            .insert(db).await?;
        RecordItemLink::from(
            (rec.id.unwrap() as i32, item.id.unwrap() as i32))
            .insert(db).await?;
        Ok(rec)
    }

    // Create new item, insert it into DB, and and insert RecItemLink into DB
    // for Record already in database
    pub async fn add_existing_item(self, db: &Db, item: Item)
        -> sqlx::Result<Self> 
    {
        let rec = match self.id {
            Some(_id) => self.clone(),
            None => self.insert(db).await?,
        };
        let item = match item.id {
            Some(_id) => item.clone(),
            None => item.insert(db).await?,
        };
        RecordItemLink::from(
            (rec.id.unwrap() as i32, item.id.unwrap() as i32))
            .insert(db).await?;
        Ok(rec)

    }

    pub async fn delete_by_id(db: &Db, id:  i32) -> sqlx::Result<i32> {
        let res = sqlx::query(
            "DELETE FROM Records WHERE id=$1 RETURNING id")
            .bind(id)
            .fetch_one(&db.pool).await?;
        let id = res.get("id");
        Ok(id)
    }

    pub async fn update_by_id(self, db: &Db, id: i32) -> sqlx::Result<i32> {
        let res = sqlx::query(
            "DELETE FROM Records WHERE id=$1 RETURNING id")
            .bind(id)
            .fetch_one(&db.pool).await?;
        let id = res.get("id");
        Ok(id)
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

