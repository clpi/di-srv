use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use sqlx::{types::chrono::{Utc, DateTime}, FromRow, Type};

/*
#[derive(Serialize, Deserialize)]
pub struct UserLogin { pub username: String, pub password: String }

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        email: &str,
        username: &str,
        password: &str,
    ) -> User {
        User { 
            id: None, 
            email: email.to_string(), 
            username: username.to_string(), 
            password: password.to_string(),
            created_at: Utc::now(),
        }
    }
}

#[async_trait]
impl Default for User {
    fn default() -> Self {
        User {
            id: None,
            username: String::from(""),
            email: String::from(""),
            password: String::from(""),
            created_at: Utc::now(),
        }   
    }
}
*/
