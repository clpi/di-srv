pub mod db;
pub use db::*;

use serde::{Serialize, Deserialize};
use sqlx::{
    prelude::*, Any, AnyPool,
};
use sqlx::{SqlitePool, sqlite::*};
use sqlx::{PgPool, postgres::{Postgres, PgPoolOptions, PgRow}}; 

#[cfg(feature="sqlite")]
pub async fn run_pg() -> () {}

#[cfg(feature="pg")]
pub async fn run_sqlite() -> () {}


#[cfg(test)]
pub mod test {
    use super::*;

    fn can_insert_user() {

    }

    fn can_delete_user() {

    }

    fn can_get_user() {

    }

    fn can_get_all_users() {

    }
}
