pub mod group;
pub mod user;
pub mod record;
pub mod item;
pub mod field;

use super::{Model, Status, Visibility, relation::Relation,};
use crate::Db;
use sqlx::{prelude::*, postgres::*, types::chrono::{DateTime, Utc}};
use serde::{Serialize, Deserialize};

//TODO consolidate some of these structs/traits

pub enum LinkType {
    GroupGroup,
    GroupUser,
    GroupRecord,
    UserUser,
    UserRecord,
    UserItem,
    RecordRecord,
    RecordItem,
    ItemItem,
    ItemField,
    FieldField,
}


#[async_trait::async_trait]
pub trait LinkModel : Default + Sized {
}

#[async_trait::async_trait]
pub trait LinkedTo<T: Model + 'static> : Model { 

    fn link_table() -> String {
        LinkType::from((Self::table(), T::table())).into()
    }
    
    async fn get_links(self, db: &Db) -> sqlx::Result<Vec<PgRow>> {
        let rows: Vec<PgRow> = sqlx::query(
           "SELECT *
            FROM $1 l INNER JOIN $2 link ON l.id=link.$3
            INNER JOIN $4 r ON link.$5=r.id AND r.id=$1")
            .bind(T::table())
            .bind(Self::link_table())
            .bind(T::foreign_id())
            .bind(Self::table())
            .bind(Self::foreign_id())
            .bind(self.id())
            .fetch_all(&db.pool).await?;
        Ok(rows)
    }

    // NOTE special case, won't work for ItemItemRels or FieldField ex.
    //      since foreign id won't be specified as such for either, so needs
    //      to be manually implemented for these cases or put in a condiitonal here
    async fn get_links_from_id(db: &Db, id: i32) -> sqlx::Result<Vec<PgRow>> {
        let (fid1, fid2): (String, String);
        if Self::foreign_id() == T::foreign_id() { 
            fid1= format!("{}1", Self::foreign_id());
            fid2= format!("{}2", T::foreign_id());
        } else {
            fid1= Self::foreign_id();
            fid2= T::foreign_id();
        }
        let res: Vec<PgRow> = sqlx::query(
           "SELECT *
            FROM $1 l INNER JOIN $2 link ON l.id=link.$3
            INNER JOIN $4 r ON link.$5=r.id AND r.id=$1")
            .bind(T::table())
            .bind(Self::link_table())
            .bind(fid2)
            .bind(Self::table())
            .bind(fid1)
            .bind(id)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    async fn linked_to_id_of_other(db: &Db, id: i32) -> sqlx::Result<Vec<PgRow>> {
        let (fid1, fid2): (String, String);
        if Self::foreign_id() == T::foreign_id() { 
            fid1= format!("{}1", Self::foreign_id());
            fid2= format!("{}2", T::foreign_id());
        } else {
            fid1= Self::foreign_id();
            fid2= T::foreign_id();
        }
        let res: Vec<PgRow> = sqlx::query(
           "SELECT *
            FROM $1 r INNER JOIN $2 link ON l.id=link.$3
            INNER JOIN $4 r ON link.$5=r.id AND r.id=$1")
            .bind(Self::table())
            .bind(Self::link_table())
            .bind(fid1)
            .bind(T::table())
            .bind(fid2)
            .bind(id)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }
}

pub struct Link(Option<i32>, Option<i32>);

impl Link {

    pub fn new(id1: Option<i32>, id2: Option<i32>) -> Self { Self(id1, id2) }

    pub fn check_foreign_id<T: LinkedTo<U> + 'static, U: LinkedTo<T> + 'static>() -> (String, String) {
        let (mut fid1, mut fid2): (String, String) = (T::foreign_id(), U::foreign_id());
        if fid1 == fid2 { 
            fid1= format!("{}1", fid1);
            fid2= format!("{}2", fid2);
        } 
        (fid1, fid2)
    }

    pub async fn insert<'a, T: LinkedTo<U> + 'static, U: LinkedTo<T> + 'static>(self, db: &Db) -> sqlx::Result<i32> {
        let table: String = LinkType::from((T::table(), U::table())).into();
        let (fid1, fid2) = Self::check_foreign_id::<T, U>();
        let res: i32 = sqlx::query
            ("INSERT INTO $1 ($2, $3, relation, status, created_at)
            VALUES ($4, $5, $6, $7) RETURNING id")
            .bind(table)
            .bind(fid1)
            .bind(fid2)
            .bind(self.0)
            .bind(self.1)
            .bind(Relation::default_for::<T, U>()) //implement
            .bind(Status::default())
            .bind(Utc::now())
            .fetch_one(&db.pool).await?
            .get("id");
        Ok(res as i32)
    }

    pub async fn insert_relation<'a, T: LinkedTo<U> + 'static, U: LinkedTo<T> + 'static>(self, db: &Db, relation: String) -> sqlx::Result<i32> {
        let table: String = LinkType::from((T::table(), U::table())).into();
        let (fid1, fid2) = Self::check_foreign_id::<T, U>();
        let res: i32 = sqlx::query
            ("INSERT INTO $1 ($2, $3, relation, status, created_at)
            VALUES ($4, $4, $6) RETURNING id")
            .bind(table)
            .bind(fid1)
            .bind(fid2)
            .bind(self.0)
            .bind(self.1)
            .bind(relation) //implement
            .bind(Status::default())
            .bind(Utc::now())
            .fetch_one(&db.pool).await?
            .get("id");
        Ok(res as i32)
    }
}

// NOTE relation schema is as follows:
// ex. For a provider-given Record-Item relation:: div::rel::rec::item::HasA
//     For a custom user-made Item-Item relation: user::<user>::rel::item::item::NeedsA
//     For a user-made status: user::<user>::status::<Status>
//     For a user-made attribute: user::<user>::attr::<Attribute>
//     For a user-made field: user::<user>>::field::<Field>

impl From<LinkType> for String {

    fn from(linktype: LinkType) -> String {
        use LinkType::*;
        let table = match linktype {
            GroupGroup => "GroupGroupLinks",
            GroupUser => "GroupUserLinks",
            GroupRecord => "GroupRecordLinks",
            UserUser => "UserUserLinks",
            UserRecord => "UserRecordLinks",
            UserItem => "UserItemLinks",
            RecordRecord => "RecordRecordLinks",
            RecordItem => "RecordItemLinks",
            ItemItem => "ItemItemLinks",
            ItemField => "ItemFieldLinks",
            _ => "",    
        };
        table.to_string()
    }
}

impl<T: Model, U: Model> From<(&T, &U)> for LinkType {

    fn from((m1, m2): (&T, &U)) -> Self {
        use LinkType::*;
        match (T::table().as_str(), U::table().as_str()) {
            ("Group", "Group") => GroupGroup,
            ("Group", "User") => GroupUser,
            ("Group", "Record") => GroupRecord,
            ("User", "User") => UserUser,
            ("User", "Record") => UserRecord,
            ("User", "Item") => UserItem,
            ("Item", "Item") => ItemItem,
            ("Item", "Field") => ItemField,
            ("Rule", "Rule") => ItemField,
            _ => GroupGroup,
        }
    }
}

impl From<(String, String)> for LinkType {

    fn from((m1, m2): (String, String)) -> Self {
        use LinkType::*;
        match (m1.as_str(), m2.as_str()) {
            ("Group", "Group") => GroupGroup,
            ("Group", "User") => GroupUser,
            ("Group", "Record") => GroupRecord,
            ("User", "User") => UserUser,
            ("User", "Record") => UserRecord,
            ("User", "Item") => UserItem,
            ("Item", "Item") => ItemItem,
            ("Item", "Field") => ItemField,
            ("Rule", "Rule") => ItemField,
            _ => GroupGroup,
        }
    }
}
