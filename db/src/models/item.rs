use serde::{Serialize, Deserialize};
use sqlx::{ prelude::*,
    types::{
        chrono::{Utc, DateTime, NaiveDate, NaiveDateTime}, uuid::{Uuid, Variant},
    }, 
    FromRow, Type, postgres::{Postgres, PgRow}, Decode
};
use crate::{
    Db, 
    models::{Model, User, Record, Status, Visibility, Priority, Field, Group,
        link::{LinkedTo, Link},
        types::{FieldType, FieldDisplay},
}};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Item {
    #[serde(default="Uuid::new_v4")]
    pub id: Uuid,
    pub uid: Uuid,
    pub name: String,
    #[serde(default="Status::default")]
    pub status: Status,
    #[serde(default="Visibility::default")]
    pub visibility: Visibility,
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
              V: Into<Status>, W: Into<Visibility> {
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

    pub async fn insert(self, db: &Db) -> sqlx::Result<Self> {
        let res: Uuid = sqlx::query(
            "INSERT INTO Items (uid, name, status, visibility, created_at)
             VALUES ($1, $2, $3, $4, $5) RETURNING id")
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.status)
            .bind(&self.visibility)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?
            .get("id");
        Ok( Self { id: res, ..self })
    }

    pub async fn add_new_field(
        self, db: &Db, field_name: String, field_type: Option<FieldType>,
    ) -> sqlx::Result<Uuid> 
    {
        let field: Field = Field::new(self.id, field_name, field_type)
            .insert(db).await?;
        let link = Link::new(self.id, field.id).insert::<Item, Field>(db).await?;
        Ok(link)
    }

    pub async fn add_existing_field(db: &Db, iid: Uuid, field: Field) -> sqlx::Result<Uuid> {
        let link = Link::new(iid, field.id).insert::<Item, Field>(db).await?;
        Ok(link)
    }

    pub async fn add_to_record(self, db: &Db, rid: Uuid) -> sqlx::Result<u32> {
        Link::new(self.id, rid).insert::<Record, Item>(db).await?;
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
            uid: Uuid::new_v4(),
            name: String::new(),
            status: Status::Active,
            visibility: Visibility::Private,
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

impl From<(User, Record)> for Item {
    fn from((user, record): (User, Record)) -> Self {
        Self::default()
    }
}

impl From<&'static PgRow> for Item {
    fn from(row: &'static PgRow) -> Self {
        Item::from_row(row).unwrap()
    }
}

#[async_trait::async_trait]
impl Model for Item {
    fn table() -> String { String::from("Items") }
    fn foreign_id() -> String { String::from("iid") }
    fn id(self) -> Uuid { self.id }

}

impl LinkedTo<Field> for Item {}
impl LinkedTo<Record> for Item {}
impl LinkedTo<Group> for Item {}
impl LinkedTo<Item> for Item {}
impl LinkedTo<User> for Item {}
