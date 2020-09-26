use std::boxed::Box;
use serde::{Serialize, Deserialize};
use sqlx::{
    prelude::*, Any, AnyPool,
    types::chrono::{DateTime, Utc},
};
use sqlx::{SqlitePool, sqlite::*};
use sqlx::{PgPool, postgres::{Postgres, PgPoolOptions, PgRow}}; 
use sqlx::postgres::*;

pub async fn init() -> sqlx::Result<()> {
    let db = Db::new().await?;
    let user = User { 
        id: None, 
        username: "test".to_string(), 
        password: "d".to_string(), 
        email: "test@div.is".to_string(), 
        created_at: Utc::now(),
    };
    add_user(&db.pool, user).await?;
    let users: Vec<User> = get_all_users(&db.pool).await?;
    for user in users.into_iter() {
        println!("User: {}", user.username);
    }
    Ok(())
}

pub struct Db {
    pub pool: sqlx::postgres::PgPool,
}

impl Db {

    pub async fn new() -> sqlx::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&dotenv::var("DATABASE_URL").unwrap()).await?;
        Ok( Self { pool } )
    }
}

pub struct BoxedModel<T>(T);
impl<T: Model> BoxedModel<T> {
    pub fn new(model: T) -> Box<T> { Box::new(model) }
}
pub trait QueryBuilder {
    type Model1;
    type Model2;
}

pub struct JoinQuery<T, U> {
    pub model1: T,
    pub model2: U,
}

impl<T, U> QueryBuilder for JoinQuery<T, U> 
where T: Model, U: Model {
    type Model1 = T;
    type Model2 = U;
    
}

pub trait Model: Sized+ Default{
    //type Item;
    fn get_by_id(db: &Db, id: i32) -> sqlx::Result<Self> { 
        Ok(Self::default()) 
    }
    fn table() -> String { String::from("") }
    fn create(db: &Db) -> sqlx::Result<u32> { Ok(0) }
    fn get_all<T: Model>(&self, db: &Db, model: T) -> sqlx::Result<Vec<T>>;
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    id: Option<i32>,
    email: String,
    username: String,
    password: String,
    created_at: DateTime<Utc>,
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
