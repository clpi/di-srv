use serde::{Serialize, Deserialize};
use sqlx::{FromRow, types::chrono::{DateTime, Utc}};
use crate::{
    models::{User, Record, Model, link::{Link, LinkInfo}},
    Db,
};

pub struct ItemEntry {

}

pub struct FieldEntry {

}
