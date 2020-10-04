use sqlx::{FromRow, types::chrono::{DateTime, Utc}};
use serde::{Serialize, Deserialize};
use crate::models::{user::User, Status, Visibility, Priority};

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

impl Record {

    pub fn new<T>(name: T, user: User) -> Self where T: Into<String> {
        Self { name: name.into(), ..Self::from(user) }
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
