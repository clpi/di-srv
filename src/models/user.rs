pub use div_db::models::{Record, User, Item};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserIn {
    id: Uuid,
    email: String,
    username: String,
}

impl From<User> for UserIn {
    fn from(user: User) -> Self {
        UserIn { id: user.id, email: user.email, username: user.username }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserQuery {
    id: Option<Uuid>,
    username: Option<String>,
    email: Option<String>
}

