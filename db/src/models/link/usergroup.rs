use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}, prelude::*};
use crate::{
    Db, models::{
        Record, Item, Group, User, 
        types::{GroupRole, Status},
    }, 
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct UserGroupLink {
    id: Option<i32>,
    uid: i32,
    gid: i32,
    group_role: GroupRole,
    status: Status,
    #[serde(default="Utc::now")]
    created_at: DateTime<Utc>,

}

impl Default for UserGroupLink {
    fn default() -> Self {
        Self {
            group_role: GroupRole::Member,
            created_at: Utc::now(),
            ..Default::default()
        }
    }
}

impl From<(i32, i32)> for UserGroupLink {
    fn from((uid, gid): (i32, i32)) -> Self {
        Self { uid, gid, ..Self::default() }
    }
}
