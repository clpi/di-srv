use crate::models::Model;
use crate::db::Db;
use sqlx::{Postgres, prelude::*, postgres::PgRow, FromRow};

pub struct Query<T> {
    model: T,
}

impl<T: Model> Query<T> {
    
    pub fn new(model: T) -> Self {
        Query { model }
    }

    pub async fn get_by_id(self, db: &Db, id: i32) -> sqlx::Result<PgRow> {
        let res: PgRow = sqlx::query("SELECT * FROM $1 WHERE id = $2")
            .bind(T::table())
            .bind(id)
            .fetch_one(&db.pool)
            .await?;
        Ok(res)
    }

    pub async fn get_all(db: &Db) -> sqlx::Result<Vec<PgRow>> {
        let res = sqlx::query("SELECT * FROM $1") 
            .bind(T::table())
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }

}

impl<User> From<User> for Query<User> {
    fn from(user: User) -> Self {
        Query { model: user }
    }
}

/*
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
*/
