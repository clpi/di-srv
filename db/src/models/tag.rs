use sqlx::{FromRow, types::chrono::{DateTime, Utc}, prelude::*, Postgres, postgres::PgRow};
use serde::{Serialize, Deserialize};
use crate::{Db, 
    models::{Model, User, Status, Visibility, Priority, Item, Group,
        link::{LinkedTo, Link},
    },
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Tag {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<Uuid>,
    pub rid: Uuid,
    pub iid: Uuid,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct ItemTag {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<Uuid>,
    pub rid: Uuid,
    pub iid: Uuid,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct RecordTag {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<Uuid>,
    pub rid: Uuid,
    pub fid: Uuid,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct FieldTag{
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<Uuid>,
    pub tid: Uuid,
    pub fid: Uuid,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}
