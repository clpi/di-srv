use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use dynomite::{self, Item as DItem, Attribute, FromAttributes, attr_map, AttributeValue, AttributeError};
use div_cloud::dynamo::DynamoClient;
use sqlx::{
    types::{chrono::{Utc, DateTime, NaiveDate, NaiveDateTime}, Json, uuid::Uuid},
    FromRow, Type, postgres::{Postgres, PgRow}, Decode, prelude::*,
};
use crate::{ Db,
    models::{Model, User}
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct UserInfo {
    // #[dynomite(default)]
    #[serde(default="Uuid::new_v4")]
    pub id: Uuid,
    // #[dynomite(partition_key)]
    pub uid: Uuid,
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

impl UserInfo {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn update<T, U>(mut self, field: T, val: U) -> Self
        where T: Into<String> {
        let field: String = field.into();
        match field.as_str() {
            "first_name" => (),
            "last_name" => (),
            "mid_initial" => (),
            "phone_number" => (),
            "occupation" => (),
            "bio" => (),
            "img_path" => (),
            "gender" => (),
            "birth_date" => (),
            "city" => (),
            "zip_code" => (),
            "state" => (),
            "country" => (),
            "social_links" => (),
            "experience" => (),
            _ => (),
        }
        self.updated_at = Utc::now();
        self.to_owned()
    }

    pub async fn get_all(db: &Db) -> sqlx::Result<Vec<Self>> {
        let res: Vec<UserInfo> = sqlx::query_as::<Postgres, UserInfo>
            ("SELECT * FROM UserInfo")
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }

    // pub async fn insert_dynamo(self, db: &DynamoClient) -> Result<(), String> {
    //     db.insert("diuser", self).await
    // }

    pub async fn insert_empty(db: &Db, uid: i32) -> sqlx::Result<i32> {
        let res = sqlx::query("INSERT INTO UserInfo (
            uid, first_name, mid_initial, last_name, phone_number,
            occupation, bio, img_path, gender, birth_date, city,
            zip_code, state, country, social_links, experience,
            user_type, updated_at ) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(vec![-1_i32])
            .execute(&db.pool).await?;
        Ok(0)
    }

    pub async fn insert_field<T: Into<String>>(self, db: &Db, field: &str, value: T) -> sqlx::Result<i32> {
        let str_fields = vec!["first_name, last_name, phone_number, occupation, bio,
            img_path, gender, birth_date, city, zip_code, state, country, experience,
            user_type"];
        if str_fields.contains(&field) {
            let res = sqlx::query("INSERT INTO UserInfo $1 VALUES $2 RETURNING id")
                .bind(value.into())
                .execute(&db.pool)
                .await?
                .rows_affected();
            Ok(res as i32)
        } else { Err(sqlx::Error::ColumnNotFound(field.to_string())) }
    }
}

impl Default for UserInfo {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            user_type: UserType::default(),
            updated_at: Utc::now(),
            experience: 0_i32,
            country: String::new(),
            ..Default::default()
        }
    }
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self { uid: user.id, ..Self::default() }
    }
}

#[derive(Serialize, Deserialize, Type, PartialEq, Clone, Default)]
pub struct SocialLinks(HashMap<String, String>);

pub struct SocialLink {
    pub label: String,
    pub url: String,
}

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

impl UserType {

}

impl Into<String> for UserType {
    fn into(self) -> String {
        match self {
            UserType::Administrator => "admin".to_string(),
            UserType::Associate => "associate".to_string(),
            UserType::Moderator => "mod".to_string(),
            UserType::User => "user".to_string(),
        }
    }
}

impl From<&'static PgRow> for UserInfo {
    fn from(row: &'static PgRow) -> Self {
        UserInfo::from_row(row).expect("Couldn't map to UserInfo")
    }
}

impl Default for UserType {
    fn default() -> Self { UserType::User }
}

impl Model for UserInfo {
    fn table() -> String { "UserInfo".to_string() }
    fn foreign_id() -> String {
        "uiid".to_string()
    }
    fn id(self) -> Uuid { self.id }
}
