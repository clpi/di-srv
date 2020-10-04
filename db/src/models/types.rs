use serde::{Serialize, Deserialize};
use sqlx::{
    types::chrono::{Utc, DateTime}, 
    FromRow, Type, postgres::{Postgres, PgRow}, Decode
};

use crate::models::Model;

/// Re-exports
pub use crate::models::{
    field::{FieldType, FieldDisplay},
    group::GroupRole,
    userinfo::UserType,
    logic::{
        condition::ConditionType,
        rule::RuleType,
        action::ActionType,
    }
};

pub struct Id<M>{
    id: Option<i32>,
    model: std::rc::Weak<M>,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq)]
#[sqlx(rename_all="snake_case")]
pub enum Status  {
    Active,
    Archived,
    Completed,
    Deleted,
    Paused,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq)]
#[sqlx(rename_all="snake_case")]
pub enum Visibility {
    Private,
    InviteOnly,
    MutualsOnly,
    Public,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq)]
#[repr(i32)]
pub enum Priority {
    Unset = 0,
    Lowest = 1,
    Low = 2,
    Medium = 3,
    High = 4,
    Highest = 5,
}

impl<T: Into<&'static str>> From<T> for Status {
    fn from(string: T) -> Self {
        match  string.into() {
            "active" => Status::Active,
            "archived"=> Status::Archived,
            "completed" => Status::Completed,
            "deleted" => Status::Deleted,
            "paused" => Status::Paused,
            _ => Status::default(),
        }
    }
}

impl<T: Into<&'static str>> From<T> for Visibility {
    fn from(string: T) -> Self {
        match  string.into() {
            "private" => Visibility::Private,
            "invite_only"=> Visibility::InviteOnly,
            "mutuals_only" => Visibility::MutualsOnly,
            "public" => Visibility::Public,
            _ => Visibility::default(),
        }
    }
}

impl From<i32> for Priority {
    fn from(priority: i32) -> Self {
        match  priority {
            0 => Priority::Unset,
            1 => Priority::Lowest,
            2 => Priority::Low,
            3 => Priority::Medium,
            4 => Priority::High,
            5 => Priority::Highest,
            _ => Priority::default(),
        }
    }
}

impl From<i32> for Id { fn from(id: i32) -> Self { Id(Some(id)) } }
impl From<Option<i32>> for Id { fn from(id: Option<i32>) -> Self { Id(id) } }

impl Default for Status { 
    fn default() -> Self { Status::Active } 
}

impl Default for Visibility { 
    fn default() -> Self { Visibility::Private } 
}

impl Default for Priority { 
    fn default() -> Self { Priority::Unset } 
}
