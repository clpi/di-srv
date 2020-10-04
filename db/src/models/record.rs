use std::rc::Weak;
use sqlx::{FromRow, types::chrono::{DateTime, Utc}};
use serde::{Serialize, Deserialize};
use crate::{
    db::Db,
    models::{user::User, Status, Visibility, Priority,  item::Item,}
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Record {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub name: String,
    pub status: String,
    pub private: bool,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

pub struct RecordObj {
    pub id: Option<i32>,
    pub user: Weak<User>,
    pub items: Vec<Weak<Item>>,
}

impl Record {

    pub fn new<T>(name: T, user: User) -> Self where T: Into<String> {
        Self { name: name.into(), ..Self::from(user) }
    }
    
    pub async fn insert(self, db: &Db) -> sqlx::Result<u32> {
        Ok(0 as u32)
    }

    pub async fn delete(self, db: &Db) -> sqlx::Result<u32> {
        Ok(0 as u32)
    }

    pub async fn update(self, db: &Db) -> sqlx::Result<u32> {
        Ok(0 as u32)
    }

    pub async fn get_by_id(self, db: &Db) -> sqlx::Result<u32> {
        Ok(0 as u32)
    }
}

impl Default for Record {
    fn default() -> Self {
        Self { 
            id: None, 
            uid: -1, 
            name: String::new(), 
            status: String::from("active"),
            private: true,
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
