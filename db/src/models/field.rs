use std::fs::File;
use crate::{models::{Visibility, Item, link::ItemFieldLink}, Db};
use serde::{Serialize, Deserialize};
use chrono::Duration;
use sqlx::{
    types::chrono::{Utc, DateTime, NaiveDateTime, NaiveDate, NaiveTime}, 
    FromRow, Type, postgres::{Postgres, PgRow}, Decode, prelude::*,
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Field { 
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub name: String,
    #[serde(default="FieldType::default")]
    pub field_type: FieldType,
    pub value: Vec<u8>,
    #[serde(default="Visibility::default")]
    pub visibility: Visibility,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Field {

    pub fn build<T, U>(uid: T, name: U) -> Field 
    where T: Into<i32>, U: Into<String> {
        Field { uid:  uid.into(), name: name.into(), ..Self::default() }
    }

    pub fn new<T, U> (uid: T, name: U) 
        -> Field where T: Into<i32>, U: Into<String>
    {
        Field { 
            uid:  uid.into(), 
            name: name.into(), 
            field_type: FieldType::Text,
            visibility: Visibility::Private,
            ..Self::default() }
    }

    pub fn with_visibility<T: Into<Visibility>>(&mut self, visibility: T) -> Self {
        Self { visibility: visibility.into(), ..self.to_owned() }
    }

    pub fn with_field_type(&mut self, field_type: FieldType) -> Self {
        Self {  field_type, ..self.to_owned() }
    }

    pub async fn insert(mut self, db: &Db) -> sqlx::Result<Self> {
        let res = sqlx::query(
            "INSERT INTO Fields 
            (name, field_type, value, visibility, created_at) 
            VALUES ($1, $2, $3, $4, $5)")
            .bind(&self.name)
            .bind(&self.field_type)
            .bind(&self.value)
            .bind(&self.visibility)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?;
        self.id = res.get("id");
        Ok(self.to_owned())
    }

    pub async fn add_to_item(self, db: &Db, iid: i32) -> sqlx::Result<u32> {
        let field = match self.id {
            Some(id) => self.clone(),
            None => self.insert(db).await?,
        };
        let link = ItemFieldLink::from(
            (field.id.expect("Item ID not set"), iid))
            .insert(db).await?;
        Ok(link)
    }

    pub fn is_in_db(self) -> bool { self.id.is_some() }
}

pub struct FieldBuilder {

}

pub struct FieldEntry {

}

impl Default for Field {
    fn default() -> Self {
        Self {
            id: None,
            uid: -1,
            name: String::new(),
            field_type: FieldType::default(),
            value: Vec::new(),
            visibility: Visibility::Private,
            created_at: Utc::now(),
        }
    }
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq)]
#[sqlx(rename_all="snake_case")]
pub enum FieldType {
    Boolean, /// Action done/not done
    IntValue, /// Hours slept
    PreciseValue, /// Money
    IntRange, /// Priority
    PreciseRange, /// Blood sugar target
    Text, /// Personal diary entry
    EnumSingle, /// .
    EnumMultiple, /// Hygienic tasks performed
    MultipleDynamic, /// .
    DateTime,
    Date,
    Duration,
    Place,
    File,
}

///TODO implement
impl From<Vec<u8>> for FieldType {
    fn from(blob: Vec<u8>) -> Self {
        Self::default()
    }
}


#[derive(sqlx::Type, Serialize, Deserialize, Clone)]
#[sqlx(rename_all="snake_case")]
pub enum FieldDisplay {
    InputLine,
    InputBox,
    ComboboxSingular,
    ComboboxMultiple,
    RadioButton,
    Checkbox,
    SliderSingleTick,
    SliderMultiTick,
    Spinner,
    Button,
    Tagbox,
    TimePicker,
    TimePickerMulti,
    DayPicker,
    DayPickerMulti,
    LocationPicker,
    FileBrowser,
}

impl From<FieldDisplay> for Vec<FieldType> {
    fn from(field_display: FieldDisplay) -> Vec<FieldType> {
        use self::{FieldType::*, FieldDisplay::*};
        match field_display {
            Button => vec![Boolean, EnumSingle,],
            Checkbox => vec![EnumMultiple,] ,
            ComboboxSingular => vec![EnumSingle, Boolean] ,
            ComboboxMultiple => vec![EnumMultiple] ,
            DayPicker => vec![Date] ,
            DayPickerMulti => vec![Duration] ,
            FileBrowser => vec![File] ,
            InputLine => vec![Text, IntValue, PreciseValue] ,
            InputBox => vec![Text] ,
            LocationPicker => vec![Place] ,
            RadioButton => vec![EnumSingle, Boolean,] ,
            SliderSingleTick => vec![IntRange, PreciseRange] ,
            SliderMultiTick => vec![IntRange, PreciseRange] ,
            Spinner => vec![IntValue, PreciseValue] ,
            Tagbox => vec![MultipleDynamic, EnumMultiple] ,
            TimePicker => vec![DateTime] ,
            TimePickerMulti => vec![Duration] ,
            _ => { vec![] }
        }
    }
}

impl From<FieldType> for Vec<FieldDisplay> {
    fn from(field_type: FieldType) -> Vec<FieldDisplay> {
        use self::{FieldType::*, FieldDisplay::*};
        match field_type {
            Boolean => vec![Button, RadioButton, ComboboxSingular],
            EnumSingle => vec![RadioButton, Button, ComboboxSingular,],
            EnumMultiple => vec![ComboboxMultiple, Checkbox, Tagbox,],
            Text => vec![InputLine, InputBox,],
            IntValue => vec![SliderSingleTick, Spinner, InputLine,],
            IntRange => vec![SliderMultiTick,],
            MultipleDynamic => vec![Tagbox],
            PreciseValue => vec![SliderSingleTick, Spinner, InputLine],
            PreciseRange => vec![SliderMultiTick,],
            Date => vec![DayPicker,],
            DateTime => vec![TimePicker,],
            Duration => vec![TimePickerMulti, DayPickerMulti],
            Place => vec![LocationPicker,],
            File => vec![FileBrowser,],
            _ => { vec![] }
        } 
    }
}

impl FieldDisplay {}

impl Default for FieldType {
    fn default() -> Self { FieldType::Text }
}

impl Default for FieldDisplay {
    fn default() -> Self { FieldDisplay::InputLine }
}


pub trait FieldItem {}
pub struct Place {
    town: String,
    province: String,
    country: String,
}

pub enum FieldTypeMap<T> where T: FieldItem {
    Boolean(bool),
    IntValue(i32),
    PreciseValue(f32),
    IntRange((i32, i32)),
    PreciseRange((f32, f32)),
    Text(String),
    EnumSingle(T),
    EnumMultiple(Vec<T>),
    MultipleDynamic(Vec<T>),
    DateTime(NaiveDateTime),
    Date(NaiveDate),
    Duration(Duration),
    Place(Place),
    File(File),
}
