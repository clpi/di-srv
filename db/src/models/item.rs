use super::{Status, Visibility};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Item {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub name: String,
    pub status: i32,
    pub visibility: i32,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Item {
    pub fn new(uid: i32, name: String) -> Self {
        Self { uid, name, ..Self::default() }
    } 

    pub fn with_visibility(&mut self, visibility: Visibility) -> Self {
        Self { visibility: visibility as i32, ..self.to_owned() }
    }

    pub fn with_status(&mut self, status: Status) -> Self {
        Self { status: status as i32, ..self.to_owned() }
    }

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
