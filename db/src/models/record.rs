use sqlx::{FromRow, types::chrono::{DateTime, Utc}};
use serde::{Serialize, Deserialize};
use crate::models::user::User;

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Record {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub name: String,
    pub active: i32,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Record {

    pub fn new<T>(name: T, user: User) -> Self where T: Into<String> {
        Self { name: name.into(), ..Self::from(user) }
    }
}

impl Default for Record {
    fn default() -> Self {
        Self { 
            id: None, uid: -1, name: String::new(), active: -1, created_at: Utc::now() 
        } 
    }
}

impl From<User> for Record {
    fn from(user: User) -> Self {
        Record { uid: user.id.unwrap(), ..Record::default() }
    }
}
