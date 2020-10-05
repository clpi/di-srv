use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use sqlx::{
    types::{chrono::{Utc, DateTime, NaiveDate, NaiveDateTime}, Json},
    FromRow, Type, postgres::{Postgres, PgRow}, Decode
};
use crate::models::{Model, User};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct UserInfo { 
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub mid_initial: Option<i8>,
    pub phone_number: Option<String>,
    pub occupation: Option<String>,
    pub bio: Option<String>,
    pub img_path: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub city: Option<String>,
    pub zip_code: Option<String>,
    pub state: Option<String>,
    pub country: String,
    pub social_links: Option<Json<SocialLinks>>,
    pub experience: i32,
    pub user_type: UserType,
    #[serde(default="Utc::now")]
    pub updated_at: DateTime<Utc>,
}

impl Default for UserInfo {
    fn default() -> Self {
        Self { 
            user_type: UserType::default(),
            updated_at: Utc::now(),
            experience: 0,
            country: String::new(),
            ..Default::default()
        }
    }
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self { 
            uid: user.id.expect("User ID not set"), 
            ..Self::default()
        }
    }
}

#[derive(Serialize, Deserialize, Type, PartialEq, Clone, Default)]
pub struct SocialLinks(HashMap<String, String>);

#[derive(Type, Serialize, Deserialize, PartialEq, Clone)]
pub enum SocialProvider {
    Twitter,
    Facebook,
    LinkedIn,
    Personal,
}

impl Default for SocialProvider {
    fn default() -> Self { SocialProvider::Personal }
}

#[derive(Type, Serialize, Deserialize, PartialEq, Clone)]
pub enum UserType {
    Administrator,
    Associate,
    Moderator,
    User,
}

impl Default for UserType {
    fn default() -> Self { UserType::User }
}

impl Model for UserInfo {
    fn table() -> String { "UserInfo".to_string() }
}
