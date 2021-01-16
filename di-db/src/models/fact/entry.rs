use dynomite::{
    Attribute, DynamoDbExt, FromAttributes, AttributeValue,
    dynamodb::{DynamoDb, DynamoDbClient}
};
use crate::{Visibility, Status, db::Db};
use sqlx::{Postgres, FromRow, postgres::*};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(FromRow, dynomite::Item, Serialize, Deserialize, Clone)]
pub struct FactEntry {
    #[serde(default = "uuid::Uuid::new_v4")]
    #[dynomite(sort_key)]
    pub id: uuid::Uuid,
    #[dynomite(partition_key)]
    pub uid: uuid::Uuid,
    pub name: String,
    pub value: String,
    pub units: Option<String>,
    #[serde(default = "Visibility::default")]
    pub visibility: Visibility,
    pub attributes: Vec<String>,
    pub notes: Vec<String>,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Default for FactEntry {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            uid: uuid::Uuid::new_v4(),
            name: String::new(),
            value: "".to_string(),
            visibility: Visibility::default(),
            attributes: Vec::new(),
            notes: Vec::new(),
            units: None,
            created_at: Utc::now(),
        }
    }
}

impl FactEntry {

    pub fn new(uid: Uuid, name: String, value: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            uid,
            name, value,
            visibility: Visibility::default(),
            created_at: Utc::now(),
            ..Self::default()
        }
    }

    pub async fn insert(&self, db: &Db) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO FactEntries (id, name, value, visibility, attributes, notes, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id")
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.value)
            .bind(&self.visibility)
            .bind(&self.attributes)
            .bind(&self.notes)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?;
        Ok(())
    }
}
