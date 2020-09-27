pub mod user;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::marker::{Send, Unpin};
use crate::db::Db;
use sqlx::{types::chrono::{Utc, DateTime}, FromRow, Type, postgres::{Postgres, PgRow}, Decode};
use com::models::{user::User, record::Record};

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

#[async_trait]
pub trait Model: Sized+ Default{
    //type Item;
    async fn get_by_id(db: &Db, id: i32) -> sqlx::Result<Self> { 
        Ok(Self::default()) 
    }
    async fn create(db: &Db) -> sqlx::Result<u32> { Ok(0) }
    async fn delete_by_id(db: &Db, id: i32) -> sqlx::Result<u32> { Ok(0) }
    async fn update_by_id<T: Model + Send>(db: &Db, id: i32, new: T) 
        -> sqlx::Result<u32> { Ok(0) }
    fn table() -> String;
    async fn get_all(db: &Db) -> sqlx::Result<Vec<Self>>;
    /*
    async fn get_all(&self, db: &Db) -> sqlx::Result<Vec<Self>>{
        let res: Vec<Self> = sqlx::query_as::<Postgres, Self>("SELECT * FROM ?")
            .bind(Self::table())
            .fetch_all(&db.pool).await?;
        Ok(())
    }
    */
}
