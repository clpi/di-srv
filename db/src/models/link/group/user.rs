use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}, Postgres, prelude::*};
use crate::{
    Db, models::{
        Record, Item, Group, User, 
        types::{GroupRole, Status},
        link::Link,
    }, 
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct GroupUserLink {
    id: Option<i32>,
    uid: i32,
    gid: i32,
    relation: String,
    status: Status,
    #[serde(default="Utc::now")]
    created_at: DateTime<Utc>,
}

impl GroupUserLink {

    pub async fn insert(self, db: &Db) -> sqlx::Result<u32> {
        let res = sqlx::query(
            "INSERT INTO GroupUserLinks (uid, gid, relation, created_at)
            VALUES ($1, $2, $3) RETURNING id")
            .bind(&self.uid)
            .bind(&self.gid)
            .bind("MemberOf")
            .bind(&self.created_at);
        let res = res.fetch_one(&db.pool).await?;
        Ok(res.get("id"))
    }

    pub async fn users_linked_to_group(self, db: &Db, group_id: i32) 
        -> sqlx::Result<Vec<User>> 
    {
        let res: Vec<User> = sqlx::query_as::<Postgres, User>(
           "SELECT u.id, u.username, u.email, u.created_at, g.name, ug.group_role, ug.created_at,
            FROM Users u INNER JOIN GroupUserLinks ug ON u.id=ug.uid
            INNER JOIN Groups g ON ug.gid=g.id AND g.id=$1")
            .bind(group_id)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    pub async fn groups_linked_to_user(self, db: &Db, user_id: i32) 
        -> sqlx::Result<Vec<Group>> 
    {
        let res: Vec<Group> = sqlx::query_as::<Postgres, Group>(
        "SELECT g.id, g.name, g.created_at, g.status, u.username, 
                ug.group_role, ug.created_at,
            FROM Groups g INNER JOIN GroupUserLinks ug ON g.id=ug.gid
            INNER JOIN Users u ON u.id=ug.uid AND u.id=$1")
            .bind(user_id)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }
}


impl Default for GroupUserLink {
    fn default() -> Self {
        Self {
            created_at: Utc::now(),
            ..Default::default()
        }
    }
}


impl From<(User, Group)> for GroupUserLink {
    fn from((user, group): (User, Group)) -> Self {
        Self { 
            uid: user.id.expect("User ID not set; not in DB"),
            gid: group.id.expect("Group ID not set; not in DB"),
            ..Self::default()
        }
    }
}

impl From<(i32, i32)> for GroupUserLink {
    fn from((uid, gid): (i32, i32)) -> Self {
        Self { uid, gid, ..Self::default() }
    }
}

