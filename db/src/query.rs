use crate::models::Model;
use crate::db::Db;
use sqlx::{Postgres, prelude::*};

pub struct Query<T: Model> {
    model: T,
}

impl<T: Model> Query<T> {
    
    pub fn new(model: T) -> Self {
        Query { model }
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
