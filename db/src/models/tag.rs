use sqlx::{FromRow, 
    types::{
        chrono::{DateTime, Utc}, 
        uuid::{Uuid, Variant}
    }, 
    postgres::PgRow, prelude::*
};
use serde::{Serialize, Deserialize};
use crate::{Db, 
    models::{Model, User, Status, Visibility, Priority, Item, Group,
        link::{LinkedTo, Link},
    },
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Tag {
    #[serde(default="Uuid::new_v4")]
    pub id: Uuid,
    pub name: String,
    pub value: String,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct ItemTag {
    #[serde(default="Uuid::new_v4")]
    pub id: Uuid,
    pub tid: Uuid,
    pub iid: Uuid,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct RecordTag {
    #[serde(default="Uuid::new_v4")]
    pub id: Uuid,
    pub tid: Uuid,
    pub rid: Uuid,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct FieldTag{
    #[serde(default="Uuid::new_v4")]
    pub id: Uuid,
    pub tid: Uuid,
    pub fid: Uuid,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Tag {
    pub fn new(name: &str, val: &str)  -> Self {
        Tag { 
            name: name.into(), value: val.into(),
            ..Default::default()
        }
    }
}

impl Default for Tag {
    fn default() -> Self {
        Tag {
            id: Uuid::new_v4(),
            name: String::new(),
            value: String::new(),
            created_at: Utc::now(),
        }
    }
}
