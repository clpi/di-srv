use serde::{Serialize, Deserialize};
use crate::models::{Field, Item};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct ItemFieldLink {
    id: Option<i32>,
    iid: i32,
    fid: i32,
    created_at: DateTime<Utc>,
}

impl Default for ItemFieldLink {
    fn default() -> Self {
        Self {
            id: None,
            iid: -1,
            fid: -1,
            created_at: Utc::now(),
        }
    }
}

impl From<(Item, Field)> for ItemFieldLink {
    fn from((item, field): (Item, Field)) -> Self {
        Self { 
            iid: item.id.expect("Record has no id"),
            fid: field.id.expect("Item id not set"),
            ..Self::default()
        }
    }
}
