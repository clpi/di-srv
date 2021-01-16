use dynomite::{
    Attribute, DynamoDbExt, FromAttributes, AttributeValue,
    dynamodb::{DynamoDb, DynamoDbClient}
};
use crate::{Visibility, Status};
use sqlx::{Postgres, FromRow, postgres::*};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


#[derive(FromRow, Serialize, Deserialize, Clone,)]
pub struct FactType {
    #[serde(default = "uuid::Uuid::new_v4")]
    pub id: uuid::Uuid,
    pub uid: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub value_type: String,
    pub units: Vec<String>,
    pub attributes: Vec<String>,
    pub notes: Vec<String>,
    #[serde(default = "Visibility::default")]
    pub visibility: Visibility,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Default for FactType {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            uid: uuid::Uuid::new_v4(),
            name: String::new(),
            value_type: "text".to_string(),
            visibility: Visibility::default(),
            status: Status::default(),
            attributes: Vec::new(),
            notes: Vec::new(),
            units: Vec::new(),
            description: None,
            created_at: Utc::now(),
        }
    }
}

// impl super::Model for FactType {

// }

// impl super::Model for FactEntry {

// }
//
impl FactType {

    pub fn new(uid: uuid::Uuid, name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            uid,
            name,
            value_type: "text".to_string(),
            visibility: Visibility::default(),
            status: Status::default(),
            created_at: Utc::now(),
            ..Self::default()
        }
    }

    pub async fn insert(&self, db: &crate::db::Db) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO FactTypes (id, name, value_type, visibility, attributes, notes, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id")
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.value_type)
            .bind(&self.visibility)
            .bind(&self.attributes)
            .bind(&self.notes)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?;
        Ok(())
    }
}
