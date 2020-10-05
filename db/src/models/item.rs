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
        Self { visibility: visibility as i32, ..self.to_owned() }
    }

    pub fn with_status(&mut self, status: Status) -> Self {
        Self { status: status as i32, ..self.to_owned() }
    }

    pub async fn insert(self, db: &Db) -> sqlx::Result<u32> {
        let link = RecordItemLink::from((
            self.rid, self.id.expect("Item ID not set"))
        );
        let res: u32 = sqlx::query(
            "INSERT INTO Items (uid, name, status, visibility, created_at)
             VALUES ($1, $2, $3, $4, $5) RETURNING id")
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.status)
            .bind(&self.visibility)
            .bind(&self.created_at);
        match res.fetch_one(&db.pool).await {
            Ok(res) => match link.insert(db).await {
                Ok(link) => Ok(res.get("id")),
                Err(_) => Err("Could not insert RecordItemLink"),
            },
            Err(_) => Err("Could not insert Item")
        }
    }

    pub async fn add_new_field(self, db: &Db, field_name: String, field_type: FieldType) -> sqlx::Result<Self> {
        let field = Field::new(self.uid, field_name);
    }

    pub async fn associate_field(self, db: &Db, field: String) -> sqlx::Result<Self> {
        Ok(self)
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
            status: Status::Active as i32,
            visibility: Visibility::Private as i32,
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


#[async_trait::async_trait]
impl Model for Item {
    fn table() -> String { String::from("Items") }

    async fn insert_db(self, db: &Db) -> sqlx::Result<Self> {
        self.insert(db).await?;
        Ok(self)
    }
}
