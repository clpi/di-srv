use std::fs::File;
use crate::models::{Visibility, Item};
use serde::{Serialize, Deserialize};
use chrono::Duration;
use sqlx::{
    types::chrono::{Utc, DateTime, NaiveDateTime, NaiveDate, NaiveTime}, 
    FromRow, Type, postgres::{Postgres, PgRow}, Decode
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Field { 
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub uid: i32,
    pub field_type: FieldType,
    pub value: Vec<u8>,
    pub visibility: Visibility,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

pub struct FieldEntry {

}

impl Default for Field {
    fn default() -> Self {
        Self {
            id: None,
            uid: -1,
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
