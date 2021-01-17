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
    pub value_type: ValueType,
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
            description: None,
            value_type: ValueType::default(),
            visibility: Visibility::default(),
            status: Status::default(),
            attributes: Vec::new(),
            notes: Vec::new(),
            units: Vec::new(),
            created_at: Utc::now(),
        }
    }
}

// impl super::Model for FactType {

// }

// impl super::Model for FactEntry {

// }
//
//
//
#[derive(Serialize, Deserialize, sqlx::Type, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ValueType {
    Text,
    Integer,
    Double,
}

impl From<String> for ValueType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "text" => Self::Text,
            "integer" => Self::Integer,
            "double" => Self::Double,
            &_ => Self::Text,
        }
    }
}

impl Default for ValueType {
    fn default() -> Self {
        Self::Text
    }
}

impl FactType {

    pub fn build(uid: uuid::Uuid, name: String) -> FactTypeBuilder {
        FactTypeBuilder::new(uuid::Uuid::new_v4(), uid, name)
    }

    pub async fn insert(&self, db: &crate::db::Db) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO FactTypes (id, uid, name, value_type, units, visibility, attributes, notes, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id")
            .bind(&self.id)
            .bind(&self.uid)
            .bind(&self.name)
            .bind(&self.value_type)
            .bind(&self.units)
            .bind(&self.visibility)
            .bind(&self.attributes)
            .bind(&self.notes)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct FactTypeBuilder {
    id: uuid::Uuid,
    uid: uuid::Uuid,
    name: String,
    description: Option<String>,
    value_type: Option<ValueType>,
    units: Option<Vec<String>>,
    attributes: Option<Vec<String>>,
    status: Option<Status>,
    notes: Option<Vec<String>>,
    visibility: Option<Visibility>,
}
impl FactTypeBuilder {

    pub fn new(id: uuid::Uuid, uid: uuid::Uuid, name: String) -> Self {
        Self { name, id, uid, ..Default::default() }
    }

    pub fn visibility<V: Into<Visibility>>(mut self, visibility: V) -> Self {
        self.visibility = Some(visibility.into());
        return self;
    }

    pub fn status<S: Into<Status>>(mut self, status: S) -> Self {
        self.status = Some(status.into());
        return self;
    }

    pub fn attribute<S: Into<String>>(mut self, attribute: S) -> Self {
        if let Some(mut attrib) = self.attributes {
            attrib.push(attribute.into());
            self.attributes = Some(attrib);
        } else {
            self.attributes = Some(vec![attribute.into()]);
        }
        return self;
    }

    pub fn attributes(mut self, attributes: Vec<String>) -> Self {
        self.attributes = Some(attributes);
        return self;
    }

    pub fn unit<S: Into<String>>(mut self, unit: S) -> Self {
        if let Some(mut units) = self.units {
            units.push(unit.into());
            self.units = Some(units);
        } else {
            self.units = Some(vec![unit.into()]);
        }
        return self;
    }

    pub fn units(mut self, units: Vec<String>) -> Self {
        self.units = Some(units);
        return self;
    }

    pub fn note<S: Into<String>>(mut self, note: S) -> Self {
        if let Some(mut notes) = self.notes {
            notes.push(note.into());
            self.notes = Some(notes);
        } else {
            self.notes = Some(vec![note.into()]);
        }
        return self;
    }

    pub fn notes(mut self, notes: Vec<String>) -> Self {
        self.notes = Some(notes);
        return self;
    }

    pub fn description<S: Into<String>>(mut self, desc: S) -> Self {
        self.description = Some(desc.into());
        return self;
    }


    pub fn value_type<V: Into<ValueType>>(mut self, kind: V) -> Self {
        self.value_type = Some(kind.into());
        return self;
    }

    pub fn buld(self) -> FactType {
        FactType {
            id: self.id,
            uid: self.uid,
            name: self.name,
            description: self.description,
            value_type: self.value_type.unwrap_or(ValueType::Text),
            attributes: self.attributes.unwrap_or_default(),
            units: self.units.unwrap_or_default(),
            status: self.status.unwrap_or_default(),
            visibility: self.visibility.unwrap_or_default(),
            notes: self.notes.unwrap_or_default(),
            created_at: Utc::now(),
        }
    }
}
