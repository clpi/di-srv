use serde::{Serialize, Deserialize};
use sqlx::{ prelude::*,
    types::{
        chrono::{Utc, DateTime, NaiveDate, NaiveDateTime}, uuid::{Uuid, Variant},
    },
    FromRow, Type, postgres::{Postgres, PgRow}, Decode
};
use crate::{
    Db,
    models::{Model, User, Record, Group, fact::{FactType, FactEntry}},
    Visibility, Status,
};

#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Item {
    #[serde(default="Uuid::new_v4")]
    pub id: Uuid,
    pub uid: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[serde(default="Status::default")]
    pub status: Status,
    #[serde(default="Visibility::default")]
    pub visibility: Visibility,
    #[serde(default="Vec::new")]
    pub attributes: Vec<String>,
    #[serde(default="Vec::new")]
    pub notes: Vec<String>,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Item {

    pub fn new<U: Into<String>>(uid: Uuid, name: U) -> Self {
        Self { uid, name: name.into(), ..Self::default() }
    }

    pub fn create<U, V, W>
        (uid: Uuid, name: U, status: V, visibility: W) -> Self
        where U: Into<String>,
              V: Into<Status>,
              W: Into<Visibility>
    {
        Self {
            name: name.into(), uid,
            status: status.into(),
            visibility: visibility.into(),
            ..Self::default()
        }
    }

    pub fn with_visibility(&mut self, visibility: Visibility) -> Self {
        Self { visibility, ..self.to_owned() }
    }

    pub fn with_status(&mut self, status: Status) -> Self {
        Self { status, ..self.to_owned() }
    }

    pub async fn get_by_id(db: &Db, id: Uuid) -> sqlx::Result<Option<Self>> {
        let res: Option<Item> = sqlx::query_as::<Postgres, Item>(
            "SELECT * FROM Items WHERE id=$1")
            .bind(id)
            .fetch_optional(&db.pool).await?;
        Ok(res)
    }

    pub async fn delete_by_id(db: &Db, id: Uuid) -> sqlx::Result<Uuid> {
        let res = sqlx::query(
            "DELETE FROM Items WHERE id=$1 RETURNING id")
            .bind(&id)
            .fetch_one(&db.pool).await?;
        Ok(res.get("id"))
    }

    pub async fn update_by_id(db: &Db, id: Uuid, item: Item) -> sqlx::Result<Option<Self>> {
        let res: Option<Item> = sqlx::query_as::<Postgres, Item>(
            "SELECT * FROM Items WHERE id=$1")
            .bind(id)
            .fetch_optional(&db.pool).await?;
        Ok(res)
    }

    pub async fn insert(&self, db: &Db) -> sqlx::Result<()> {
        let res: Uuid = sqlx::query(
            "INSERT INTO Items (uid, name, description, status, visibility, attributes, notes, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id")
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.status)
            .bind(&self.visibility)
            .bind(&self.attributes)
            .bind(&self.notes)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?
            .get("id");
        Ok(())
    }

    pub async fn add_new_fact(&self, db: &Db, fact: String, val: String) -> sqlx::Result<()>
    {
        let fact: FactEntry = FactEntry::new(self.id, fact, val);
        Ok(())
    }

    pub async fn add_existing_field(db: &Db, iid: Uuid, field: FactEntry) -> sqlx::Result<()> {
        Ok(())
    }

    pub async fn add_to_record(self, db: &Db, rid: Uuid) -> sqlx::Result<u32> {
        Ok(0)
    }

    pub async fn get_all_by_user(db: &Db, uid: Uuid) -> sqlx::Result<Vec<Item>> {
        let res: Vec<Item> = sqlx::query_as::<Postgres, Item>(
            "SELECT * FROM Items i WHERE i.uid=$1")
            .bind(uid)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    pub async fn get_all_from_record(db: &Db, rid: Uuid) -> sqlx::Result<Vec<Item>> {
        let res: Vec<Item> = sqlx::query_as::<Postgres, Item>(
            "SELECT * FROM Items i WHERE i.uid=$1") //IMPLEMENT
            .bind(rid)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }
}

pub struct ItemEntry {
    pub id: Option<i32>,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            description: None,
            uid: Uuid::new_v4(),
            name: String::new(),
            status: Status::Active,
            visibility: Visibility::Private,
            attributes: Vec::new(),
            notes: Vec::new(),
            created_at: Utc::now(),
        }
    }
}

impl From<Record> for Item {
    fn from(record: Record) -> Self {
        Self { uid: record.uid, ..Self::default() }
    }
}

impl From<User> for Item {
    fn from(user: User) -> Self {
        Self { uid: user.id, ..Default::default() }
    }
}

#[async_trait::async_trait]
impl super::Model for Item {
    fn table() -> String { String::from("Items") }
    fn foreign_id() -> String { String::from("iid") }
    fn id(self) -> Uuid { self.id }

}

