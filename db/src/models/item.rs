use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}};
use crate::{
    db::Db,
    models::{user::User, record::Record, Status, Visibility, Priority}
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Item {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub name: String,
    pub status: String,
    pub private: bool,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Item {
    pub fn new(uid: i32, name: String) -> Self {
        Self { uid, name, ..Self::default() }
    } 

    pub fn private(&mut self, private: bool) -> Self {
        Self { private, ..self.to_owned() }
    }

    pub fn with_status(&mut self, status: String) -> Self {
        Self { status, ..self.to_owned() }
    }

}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: None,
            uid: -1,
            name: String::new(),
            status: "active".to_string(),
            private: true,
            created_at: Utc::now(),
        }
    }
}

///TODO implement
impl From<Record> for Item {
    fn from(record: Record) -> Self {
        Self::default()
    }
}

///TODO implement
impl From<User> for Item {
    fn from(user: User) -> Self {
        Self::default()
    }
}

///TODO implement
impl From<(User, Record)> for Item {
    fn from((user, record): (User, Record)) -> Self {
        Self::default()
    }
}
