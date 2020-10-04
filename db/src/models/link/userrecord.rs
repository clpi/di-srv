use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}};
use crate::{
    models::{User, Record, Model, link::{Link, LinkInfo}},
    Db,
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct UserRecordLink {
    id: Option<i32>,
    uid: i32,
    rid: i32,
    created_at: DateTime<Utc>,
}

impl Default for UserRecordLink {
    fn default() -> Self {
        Self {
            id: None,
            uid: -1,
            rid: -1,
            created_at: Utc::now(),
        }
    }
}

impl From<(User, Record)> for UserRecordLink {
    fn from((user, record): (User, Record)) -> Self {
        Self { 
            uid: user.id.expect("User has no id"),
            rid: record.id.expect("Record id not set"),
            ..Self::default()
        }
    }
}

impl From<UserRecordLink> for Link<User, Record> {
    fn from(link: UserRecordLink) -> Self {
        Self {
            model1: User::default(),
            model2: Record::default(),
            info: LinkInfo::new(),
        }
    }
}

impl Link<User, Record> {

}
