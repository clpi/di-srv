use crate::{Db, models::{
    Model, User, link::UserGroupLink,
    types::{Id, Status, Visibility}
}};
use serde::{Serialize, Deserialize};
use sqlx::{
    types::chrono::{Utc, DateTime}, 
    FromRow, Type, postgres::{Postgres, PgRow}, Decode
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Group {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub name: String,
    #[serde(default="Visibility::default")]
    pub visibility: Visibility,
    #[serde(default="Status::default")]
    pub status: Status,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Group {

    pub fn new<T: Into<String>, U: Into<i32>>(name: T, uid: U) -> Self {
        Self { name: name.into(), uid: uid.into(), ..Self::default() }
    }

    pub async fn insert(self, db: &Db) -> sqlx::Result<u32> {
        let link = UserGroupLink::from(
                (self.uid, self.id.expect("Group ID not set"))
        );
        let res: u32 = sqlx::query_scalar(
            "INSERT INTO Groups (uid, name, visibility, status, created_at)
            VALUES ($1, $2, $3, $4, $5) RETURNING id")
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.visibility)
            .bind(&self.status)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?;
        Ok(res)
    }

    pub async fn add_user(self, user: User, role: GroupRole) -> sqlx::Result<u32> {
        Ok(0)
    }
}

impl Default for Group {
    fn default() -> Self {
        Self { 
            status: Status::Active,
            visibility: Visibility::Public,
            created_at: Utc::now(), ..Default::default() 
        }
    }
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq)]
#[sqlx(rename_all="snake_case")]
pub enum GroupRole {
    Admin,
    Moderator,
    Member,
}
