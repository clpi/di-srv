//pub use com::models::{user::User, record::Record};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}};
use serde::{Serialize, Deserialize};
use crate::db::Db;
use sqlx::Postgres;
use async_trait::async_trait;
use super::Model;

#[derive(Serialize, Deserialize)]
pub struct UserLogin { pub username: String, pub password: String }

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub password: String,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        email: &str,
        username: &str,
        password: &str,
    ) -> User {
        User { 
            id: None, 
            email: email.to_string(), 
            username: username.to_string(), 
            password: password.to_string(),
            created_at: Utc::now(),
        }
    }

    pub async fn get_all(db: &Db) -> sqlx::Result<Vec<User>> {
        let res: Vec<User> = sqlx::query_as::<Postgres, User>
            ("SELECT * FROM Users") 
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }

    pub async fn get_by_id(db: &Db, id: i32) -> sqlx::Result<User> {
        let res: User = sqlx::query_as::<Postgres, User>
            ("SELECT * FROM Users WHERE id=$1") 
            .bind(id)
            .fetch_one(&db.pool)
            .await?;
        Ok(res)
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            id: None,
            username: String::from(""),
            email: String::from(""),
            password: String::from(""),
            created_at: Utc::now(),
        }   
    }
}

#[async_trait]
impl Model for User {
    fn table() -> String { String::from("Users") }

    async fn get_all(db: &Db) -> sqlx::Result<Vec<Self>> {
        let res: Vec<User> = sqlx::query_as::<Postgres, User>("SELECT * FROM Users")
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

}

