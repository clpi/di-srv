pub use com::models::{user::User, record::Record};
use crate::db::Db;
use sqlx::Postgres;
use async_trait::async_trait;
use super::Model;

#[async_trait]
impl Model for User {
    fn table() -> String { String::from("Users") }

    async fn get_all(db: &Db) -> sqlx::Result<Vec<Self>> {
        let res: Vec<User> = sqlx::query_as::<Postgres, User>("SELECT * FROM Users")
            .fetch_all(&db.pool).await?;
        Ok(res)
    }
}

