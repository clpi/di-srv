use sqlx::postgres::types::PgInterval;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use div_com::DError;
use chrono::{DateTime, Utc};
use dynomite::{FromAttributes, AttributeValue, Attribute, dynamodb, DynamoDbExt, Attributes, AttributeError};

use crate::models::Model;

pub struct Id<M>{
    id: Option<i32>,
    model: std::rc::Weak<M>,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[sqlx(rename_all="snake_case")]
pub enum Status  {
    Active,
    Archived,
    Completed,
    Deleted,
    Paused,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[sqlx(rename_all="snake_case")]
pub enum Visibility {
    Private,
    InviteOnly,
    MutualsOnly,
    Public,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[repr(i32)]
pub enum Priority {
    Unset = 0,
    Lowest = 1,
    Low = 2,
    Medium = 3,
    High = 4,
    Highest = 5,
}

// pub struct Attribute(sqlx::types::Json);

// #[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
// pub struct Attribute {
//     pub id: uuid::Uuid,
//     pub uid: uuid::Uuid,
//     pub name: String,
//     pub value; option<String>,
//     pub created_at: Datetime<Utc>,
// }

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

impl From<Visibility> for String {
    fn from(vis: Visibility) -> String {
        match vis {
            Visibility::Public => "public".to_string(),
            Visibility::InviteOnly => "invite_only".to_string(),
            Visibility::MutualsOnly => "mutuals_only".to_string(),
            Visibility::Private => "private".to_string(),
        }
    }
}

impl From<Status> for String {
    fn from(status: Status) -> String {
        match status {
            Status::Archived => "archived".to_string(),
            Status::Deleted => "deleted".to_string(),
            Status::Active => "active".to_string(),
            Status::Completed => "completed".to_string(),
            Status::Paused => "paused".to_string(),
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

impl Default for Status {
    fn default() -> Self { Status::Active }
}

impl Default for Visibility {
    fn default() -> Self { Visibility::Private }
}

impl Default for Priority {
    fn default() -> Self { Priority::Unset }
}


#[async_trait::async_trait]
pub trait Datatype: Attribute {

    async fn get(&self) -> Result<(), dynamodb::PutItemError> {
        Ok(())
    }

    async fn entries_with(&self, table: &str) -> Result<(), dynamodb::PutItemError> {
        Ok(())
    }
}

impl Status {
    pub fn active() -> Self {
        Self::Active
    }
}

impl Visibility {
    pub fn public() -> Self {
        Self::Public
    }

    pub fn private() -> Self {
        Self::Private
    }
}

impl Datatype for Visibility {

}

impl std::str::FromStr for Visibility {
    type Err = DError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "public" => Ok(Self::Public),
            "private" => Ok(Self::Private),
            "mutuals_only" => Ok(Self::MutualsOnly),
            "invite_only" => Ok(Self::InviteOnly),
            _ => Err(DError::AttError(AttributeError::InvalidFormat)),
        }
    }
}

impl ToString for Visibility {
    fn to_string(&self) -> String {
        match self {
            Self::InviteOnly => "invite_only".to_string(),
            Self::Private => "private".to_string(),
            Self::Public => "public".to_string(),
            Self::MutualsOnly => "mutuals_only".to_string(),
        }
    }
}

impl Attribute for Visibility {

    fn from_attr(value: AttributeValue) -> Result<Self, AttributeError> {
        if let Some(s) = value.s {
            match Visibility::from_str(s.as_str()) {
                Ok(v) => Ok(v),
                Err(_) => Err(AttributeError::InvalidFormat),
            }
        } else {
            Err(AttributeError::InvalidFormat)
        }
    }

    fn into_attr(self: Self) -> AttributeValue {
        match self {
            _ => AttributeValue { s: Some(self.to_string()), ..Default::default() }
        }

    }
}
