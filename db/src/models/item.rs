use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}, postgres::PgRow, prelude::*};
use crate::{
    Db, 
    models::{Model, User, Record, Status, Visibility, Priority, Field, 
        link::{RecordItemLink, ItemFieldLink},
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

    pub fn new(uid: i32, name: String) -> Self {
        Self { uid, name, ..Self::default() }
    } 

    pub fn private(&mut self, visibility: Visibility) -> Self {
        Self { visibility, ..self.to_owned() }
    }

    pub fn with_status(&mut self, status: Status) -> Self {
        Self { status, ..self.to_owned() }
    }

    pub async fn insert(mut self, db: &Db) -> sqlx::Result<Self> {
        let res = sqlx::query(
            "INSERT INTO Items (uid, name, status, visibility, created_at)
             VALUES ($1, $2, $3, $4, $5) RETURNING id")
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.status)
            .bind(&self.visibility)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?;
        self.id = res.get("id");
        Ok(self)
    }

    pub async fn add_new_field(
        self, db: &Db, field_name: String, field_type: FieldType
    ) -> sqlx::Result<u32> 
    {
        let res = match self.id {
            Some(id) => self.clone(),
            None => self.insert(db).await?,
        };
        let field = Field::new(res.uid, field_name)
            .insert(db).await?;
        let link = ItemFieldLink::from((res, field))
            .insert(db).await?;
        Ok(link)
    }

    pub async fn add_existing_field(self, db: &Db, field: Field) -> sqlx::Result<u32> {
        let field = match field.id { //Checks if field has ID retrieved from DB
            Some(id) => field.clone(),
            None => field.insert(db).await?,
        };
        let link = ItemFieldLink::from((self, field))
            .insert(db).await?;
        Ok(link)
    }

    pub async fn add_to_record(self, db: &Db, rid: i32) -> sqlx::Result<u32> {
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


#[async_trait::async_trait]
impl Model for Item {
    fn table() -> String { String::from("Items") }

}
