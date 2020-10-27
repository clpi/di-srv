//TODO programmatically do this stuff with efficient code re-use. this way of implementing
//     relationships probably looks so stupid to the outside, but i guess it's more important
//     to get it working first and then optimize later!

use sqlx::{ prelude::*,
    types::{
        chrono::{Utc, DateTime, NaiveDate, NaiveDateTime}, uuid::{Uuid, Variant},
    }, 
    FromRow, Type, postgres::{Postgres, PgRow}, Decode
};
use serde::{Serialize, Deserialize};
use crate::models::{
    Group, User, Record, Item, Model, Status,
    link::{LinkType, Link, LinkedTo},
};

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, sqlx::FromRow, Clone, PartialEq)]
pub struct CustomRelation {
    pub id: Option<Uuid>,
    pub uid: Uuid,
    pub name: String,
    pub value: Option<Vec<u8>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}


// TODO have different types / implementations per linkage
#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[sqlx(rename_all="snake_case")]
pub enum Relation {
    HasA,
    CreatedBy,   
    HasMember,
    AssociatedWith,
    HasMutual,
}

impl Relation {

    fn valid<T: LinkedTo<U> + 'static, U: LinkedTo<T> + 'static>() -> Vec<impl Relationship> {
        vec![Relation::HasA,] //TODO implement
    }

    pub fn default_for<T: LinkedTo<U> + 'static, U: LinkedTo<T> + 'static>() -> impl Relationship {
        match LinkType::from((T::table(), U::table())) {
            LinkType::GroupGroup => Relation::HasA,
            LinkType::GroupUser => Relation::HasMember,
            LinkType::GroupRecord => Relation::AssociatedWith,
            LinkType::UserUser => Relation::HasMutual,
            LinkType::UserRecord => Relation::HasA,
            LinkType::UserItem => Relation::HasA,
            LinkType::ItemItem => Relation::HasA,
            LinkType::ItemField => Relation::HasA,
            LinkType::FieldField => Relation::HasA,
            _ => Relation::HasA,
        }
    }
}

pub trait Relationship : sqlx::Type<Postgres> + sqlx::Encode<'static, Postgres> {}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[sqlx(rename_all="snake_case")]
pub enum GroupUserRelation {
    HasMember,
    HasAdmin,
    HasModerator,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[sqlx(rename_all="snake_case")]
pub enum GroupGroupRelation {
    Partnered,
    Associated,
    Blocked,
    None,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[sqlx(rename_all="snake_case")]
pub enum GroupRecordRelation {
    HasA,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[sqlx(rename_all="snake_case")]
pub enum UserUserRelation {
    Mutual,
    Following,
    Blocked,
    Muted,
    None,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[sqlx(rename_all="snake_case")]
pub enum UserRecordRelation {
    Group,
    User,
    Record
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[sqlx(rename_all="snake_case")]
pub enum UserItemRelation {
    Friend,
    Blocked,
    Muted,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[sqlx(rename_all="snake_case")]
pub enum RecordRecordRelation {
    Admin,
    Member,
    Moderator,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[sqlx(rename_all="snake_case")]
pub enum RecordItemRelation {
    Admin,
    Member,
    Moderator,
    CreatedBy,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[sqlx(rename_all="snake_case")]
pub enum ItemItemRelation {
    Admin,
    Member,
    Moderator,
    CreatedBy,
}


#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[sqlx(rename_all="snake_case")]
pub enum ItemFieldRelation {
    Admin,
    Member,
    Moderator,
    CreatedBy,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, PartialEq, Copy)]
#[sqlx(rename_all="snake_case")]
pub enum FieldFieldRelation {
    Admin,
    Member,
    Moderator,
    CreatedBy,
}

impl Relationship for Relation {}
impl Relationship for GroupGroupRelation {}
impl Relationship for GroupUserRelation {}
impl Relationship for GroupRecordRelation {}
impl Relationship for UserUserRelation {}
impl Relationship for UserRecordRelation {}
impl Relationship for UserItemRelation {}
impl Relationship for RecordRecordRelation {}
impl Relationship for RecordItemRelation {}
impl Relationship for ItemItemRelation {}
impl Relationship for ItemFieldRelation {}
impl Relationship for FieldFieldRelation {}
