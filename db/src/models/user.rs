//pub use com::models::{user::User, record::Record};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}};
use serde::{Serialize, Deserialize};
use crate::{db::Db, models::Record};
use sqlx::Postgres;
use async_trait::async_trait;
use super::Model;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLogin { pub username: String, pub password: String }

#[derive(Serialize, Deserialize, Clone)]
pub struct UserRegister { email: String, username: String, password: String }

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
    pub fn new<T, U, V>(email: T, username: U, password: V,) -> User
        where T: Into<String>, U: Into<String>, V: Into<String> {
        User { 
            email: email.into(), 
            username: username.into(), 
            password: password.into(),
            ..User::default()
        }
    }

    pub async fn insert(self, db: &Db) -> sqlx::Result<()> {
        println!("INSERTING {} {} {}", &self.username, &self.email, &self.password);
        let mut conn = db.pool.acquire().await?;
        sqlx::query
            ("INSERT INTO Users (email, username, password, created_at)
              VALUES ($1, $2, $3, $4);") 
            .bind(self.email)
            .bind(self.username)
            .bind(self.password)
            .bind(Utc::now())
            .execute(&mut conn).await?;
            //.fetch_one(&db.pool).await?;
        conn.release();
        Ok(())
    }

    pub async fn delete_by_username(db: &Db, username: String) -> sqlx::Result<u32> {
        let res: u32 = sqlx::query_scalar
            ("DELETE FROM Users WHERE username=$1 RETURNING id")
            .bind(username)
            .fetch_one(&db.pool).await?;
        Ok(res)
    }

    pub async fn delete_by_id(db: &Db, id: i32) -> sqlx::Result<u32> {
        let res: u32 = sqlx::query_scalar
            ("DELETE FROM Users WHERE id=$1 RETURNING id")
            .bind(id)
            .fetch_one(&db.pool).await?;
        Ok(res)
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

    pub async fn get_by_username(db: &Db, username: String) -> sqlx::Result<User> {
        let res: User = sqlx::query_as::<Postgres, User>
            ("SELECT * FROM Users WHERE username=$1") 
            .bind(username)
            .fetch_one(&db.pool)
            .await?;
        Ok(res)
    }

    pub async fn get_all_records(self, db: &Db) -> sqlx::Result<Vec<Record>> {
        let res: Vec<Record> = sqlx::query_as::<Postgres, Record>
            ("WITH Users as u SELECT * FROM Records r WHERE r.uid = u.id AND u.id = $1")
            .bind(self.id.expect("No id set"))
            .fetch_all(&db.pool).await?;
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

#[async_trait::async_trait]
impl Model for User {
    fn table() -> String { String::from("Users") }
}

