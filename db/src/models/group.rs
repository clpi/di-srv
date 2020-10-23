use crate::{Db, models::{
    Model, User, Record, link::{Link, LinkedTo}, Item,
    types::{Id, Status, Visibility}
}};
use serde::{Serialize, Deserialize};
use sqlx::{
    types::{
        chrono::{Utc, DateTime}, uuid::{Uuid, Variant},
    }, 
    FromRow, Type, postgres::{Postgres, PgRow}, Decode
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Group {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<Uuid>,
    pub uid: Uuid,
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

    pub async fn insert(self, db: &Db) -> sqlx::Result<i32> {
        let res: i32 = sqlx::query_scalar(
            "INSERT INTO Groups (uid, name, visibility, status, created_at)
            VALUES ($1, $2, $3, $4, $5) RETURNING id")
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.visibility)
            .bind(&self.status)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?;
        let link = Link::new(Some(self.uid), self.id).insert::<Group, User>(db).await?;
        Ok(res)
    }

    pub async fn add_member(self, db: &Db, user: User) -> sqlx::Result<i32> {
        let link = Link::new(user.id, self.id).insert::<Group, User>(db).await?;
        Ok(link) //to implement
    }

    pub async fn add_admin(self, db: &Db, admin: User) -> sqlx::Result<i32> {
        let link = Link::new(admin.id, self.id).insert::<Group, User>(db).await?;
        Ok(link) //to implement
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

impl From<&'static PgRow> for Group {
    fn from(row: &'static PgRow) -> Self {
        Group::from_row(row).expect("Couldn't map to Group")
    }
}

impl Model for Group { 
    fn table() -> String { String::from("Groups") }
    fn foreign_id() -> String {
       String::from("gid") 
    }
    fn id(self) -> i32 { self.id.expect("ID not set for group") }
}
impl LinkedTo<User> for Group {}
impl LinkedTo<Record> for Group {}
impl LinkedTo<Item> for Group {}
impl LinkedTo<Group> for Group {}
