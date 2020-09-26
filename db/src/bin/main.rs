use serde::{Serialize, Deserialize};
use sqlx::{
    prelude::*, Any, AnyPool,
};
use sqlx::{SqlitePool, sqlite::*};
use sqlx::{PgPool, postgres::PgPoolOptions}; 

#[cfg(feature="sqlite")]
pub mod sqlite {
    use super::*;
    use sqlx::{SqlitePool, sqlite::*};
    pub async fn run_sqlite() -> () {}
}


#[cfg(feature="pg")]
pub mod pg {
    use super::*;
    use sqlx::{PgPool, postgres::PgPoolOptions}; 
    pub async fn run_pg() -> () {}
}

#[async_std::main]
pub async fn main() -> sqlx::Result<()> {
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dotenv::var("DATABASE_URL").unwrap()).await?;
    let user = User { 
        id: None, 
        username: "chrisp".to_string(), 
        password: "p".to_string(), 
        email: "chrisp@div.is".to_string(), 
        created_at: None };
    //add_user(&db, user).await?;
    let users: Vec<User> = get_all_users(&db).await?;
    for user in users.into_iter() {
        println!("User: {}", user.username);
    }
    Ok(())
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    id: Option<i32>,
    email: String,
    username: String,
    password: String,
    created_at: Option<i32>,
}


async fn add_user(pool: &PgPool, user: User) -> sqlx::Result<i32> {
    let res = sqlx::query!(
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

async fn get_all_users(pool: &PgPool) -> sqlx::Result<Vec<User>> {
    let res: Vec<User> = sqlx::query_as::<sqlx::Postgres, User>
        (r#"SELECT * FROM Users"#)
        .fetch_all(pool)
        .await?;
    Ok(res)
}

pub struct Db {}

impl Db {

    pub async fn init() -> () {}

}
