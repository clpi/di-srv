use std::marker::{Send, Unpin};
use std::io::*;
use crate::models::{user::User, Model};
use sqlx::{ 
    FromRow, Database,  prelude::*, error::DatabaseError,
    types::{
        chrono::{DateTime, Utc},
        uuid::Uuid, Json,
    },
    postgres::{
        Postgres, PgPool, PgRow, PgPoolOptions,
        PgDatabaseError, PgListener, PgNotification,
    },
};

// TODO implement listener/notifications for sse
#[derive(Clone)]
pub struct Db {
    pub pool: sqlx::postgres::PgPool,
}

pub async fn init() -> () {}

impl Db {

    pub async fn new() -> sqlx::Result<Self> {
        let dburl = &dotenv::var("DATABASE_URL").expect("DB URL not set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(dburl).await?;
        Ok( Self { pool } )
    }

    pub fn new_blocking() -> sqlx::Result<Self> {
        let dburl = &dotenv::var("DATABASE_URL").expect("DB URL not set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(dburl);
        let pool = async_std::task::block_on(pool);
        Ok( Self { pool: pool.unwrap() } )
    }

    pub async fn query(self, query: &str) -> sqlx::Result<()> {
        Ok(())
    }

    pub async fn init(self) -> sqlx::Result<Self> {
        (&self.pool).execute(include_str!("../sql/up.sql")).await?;
        Ok(self)
    }

    pub async fn down(self) -> sqlx::Result<Self> {
        (&self.pool).execute(include_str!("../sql/down.sql")).await?;
        Ok(self)
    }

    pub async fn clear(self) -> sqlx::Result<Self> {
        (&self.pool).execute(include_str!("../sql/clear.sql")).await?;
        Ok(self)
    }

    pub async fn exec(self, query: &str) -> sqlx::Result<()> {
        sqlx::query(query).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn clear_table(self, table: &str) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM ?;").bind(table).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn drop_table(self, table: &str) -> sqlx::Result<()> {
        sqlx::query("DROP TABLE IF EXISTS ?;").bind(table).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn conn(self) -> sqlx::Result<sqlx::pool::PoolConnection<Postgres>> {
        Ok(self.pool.acquire().await?)
    }

    //pub async fn query<'r, T>(&self, qstr: String, model: T) -> sqlx::Result<()>
    //where T: Model + sqlx::FromRow<'r, PgRow> + Send + Unpin {
        ////let res: T = sqlx::query_as::<Postgres, T>(&qstr).fetch(&self.pool).await?;
        //Ok(())
    //}
}

/*
pub async fn add_user(pool: &PgPool, user: User) -> sqlx::Result<i32> {
    let res = sqlx::query(
        r#"
            INSERT INTO Users ( email, username, password )
            VALUES ( $1, $2, $3 )
            RETURNING id
        "#,
       user.email, user.username, user.password 
    )
        .fetch_one(pool)
        .await?;
    Ok(res.id)
}

pub async fn get_all_users(pool: &PgPool) -> sqlx::Result<Vec<User>> {
    let res: Vec<User> = sqlx::query_as::<sqlx::Postgres, User>
        (r#"SELECT * FROM Users"#)
        .fetch_all(pool)
        .await?;
    Ok(res)
}
*/

#[cfg(test)]
pub mod tests {

    pub fn can_connect() {

    }
}
