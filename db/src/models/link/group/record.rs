use serde::{Serialize, Deserialize};
use crate::{ Db,
    models::{ attrib::Attrib, Record, Group, Item, Priority, Field,}, 
};
use sqlx::{FromRow, Postgres, types::chrono::{DateTime, Utc}, prelude::*};
//TODO add to up.sql

#[derive(FromRow, Serialize, Deserialize)]
pub struct GroupRecordLink {
    pub id: Option<i32>,
    pub gid: i32,
    pub rid: i32,
    pub created_at: DateTime<Utc>,
}

impl GroupRecordLink {

}

