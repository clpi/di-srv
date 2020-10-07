use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}, prelude::*};
use crate::{
    models::{User, Record, Model, link::{Link, LinkInfo, LinkModel}},
    Db,
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct UserRecordLink {
    #[serde(skip_serializing_if="Option::is_none")]
    id: Option<i32>,
    uid: i32,
    rid: i32,
    #[serde(default="Utc::now")]
    created_at: DateTime<Utc>,
}

impl Link<User, Record> {

    pub async fn create(self, db: &Db) -> sqlx::Result<i32> {
        let res = sqlx::query(
            "INSERT INTO UserRecordLinks (uid, gid, created_at)
            VALUES ($1, $2, $3) RETURNING id")
            .bind(self.0.id.unwrap())
            .bind(self.1.id.unwrap())
            .bind(Utc::now());
        let res = res.fetch_one(&db.pool).await?;
        Ok(res.get("id"))
    }
}


impl UserRecordLink {

    pub fn new(user: User, record: Record) -> Self {
        Self::from((user, record))
    }

    pub async fn insert(self, db: &Db) -> sqlx::Result<u32> {
        let res = sqlx::query
            ("INSERT INTO UserRecordLinks (uid, rid, created_at)
            VALUES ($1, $2, $3) RETURNING id")
            .bind(&self.uid)
            .bind(&self.rid)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?;
        Ok(res.get("id"))
    }

    pub async fn records_linked_to_user(db: &Db, user_id: i32) -> () {}

    pub async fn users_linked_to_record(db: &Db, record_id: i32) -> () {}

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
    fn from((User, record): (User, Record)) -> Self {
        Self { 
            uid: User.id.expect("User has no id"),
            rid: record.id.expect("Record id not set"),
            ..Self::default()
        }
    }
}

impl From<(i32, i32)> for UserRecordLink {
    fn from((uid, rid): (i32, i32)) -> Self {
        Self {  uid, rid, ..Self::default() }
    }
}

/*
impl From<Link<User, Record>> for UserRecordLink {
    fn from(link: Link<User, Record>) -> Self {
        Self{ uid: link.0.id.unwrap(), rid: link.1.id.unwrap(), ..Default::default() }
    }
}
*/

impl Link<User, Record> {

}

impl LinkModel for UserRecordLink {}
