use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}, postgres::PgRow, prelude::*};
use crate::{
    Db, 
    models::{Model, User, Record, Status, Visibility, Priority, Field, Group,
        link::{LinkedTo, Link},
        types::{FieldType, FieldDisplay},
}};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Item {
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

impl Item {

    pub fn new<T: Into<i32>, U: Into<String>>(uid: T, name: U) -> Self {
        Self { uid: uid.into(), name: name.into(), ..Self::default() }
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

    pub fn with_visibility(&mut self, visibility: Visibility) -> Self {
        Self { visibility, ..self.to_owned() }
    }

    pub fn with_status(&mut self, status: Status) -> Self {
        Self { status, ..self.to_owned() }
    }

    pub async fn insert(self, db: &Db) -> sqlx::Result<Self> {
        let res: i32 = sqlx::query(
            "INSERT INTO Items (uid, name, status, visibility, created_at)
             VALUES ($1, $2, $3, $4, $5) RETURNING id")
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.status)
            .bind(&self.visibility)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?
            .get("id");
        Ok( Self { id: Some(res), ..self })
    }

    pub async fn add_new_field(
        db: &Db, iid: i32, field_name: String, field_type: FieldType
    ) -> sqlx::Result<i32> 
    {
        let field = Field::new(field_name, field_type).insert(db).await?;
        let link = Link::new(Some(iid), field.id).insert::<Item, Field>(db).await?;
        Ok(link)
    }

    pub async fn add_existing_field(db: &Db, iid: i32, field: Field) -> sqlx::Result<i32> {
        let link = Link::new(Some(iid), field.id).insert::<Item, Field>(db).await?;
        Ok(link)
    }

    pub async fn add_to_record(self, db: &Db, rid: i32) -> sqlx::Result<u32> {
        Link::new(self.id, Some(rid)).insert::<Record, Item>(db).await?;
        Ok(0)
    }

}

pub struct ItemEntry {
    pub id: Option<i32>,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: None,
            uid: -1,
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
        Self { uid: user.id.expect("User ID not set"), ..Self::default() }
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
    fn id(self) -> i32 { self.id.expect("ID not set for Item") }

}

impl LinkedTo<Field> for Item {}
impl LinkedTo<Record> for Item {}
impl LinkedTo<Group> for Item {}
impl LinkedTo<Item> for Item {}
impl LinkedTo<User> for Item {}
