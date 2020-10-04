use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}};
use crate::models::{Record, Item};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct RecordItemLink {
    id: Option<i32>,
    rid: i32,
    iid: i32,
    created_at: DateTime<Utc>,
}

impl Default for RecordItemLink {
    fn default() -> Self {
        Self {
            id: None,
            rid: -1,
            iid: -1,
            created_at: Utc::now(),
        }
    }
}

impl From<(Record, Item)> for RecordItemLink {
    fn from((record, item): (Record, Item)) -> Self {
        Self { 
            rid: record.id.expect("Record has no id"),
            iid: item.id.expect("Item id not set"),
            ..Self::default()
        }
    }
}
