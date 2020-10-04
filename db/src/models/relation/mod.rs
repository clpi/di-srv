pub mod itemitem;

pub trait Relation {}

#[derive(sqlx::Type)]
#[sqlx(rename="item_relation", rename_all="lowercase")]
pub enum ItemRelation {
    Parent,
    Child
}

#[derive(sqlx::Type)]
#[sqlx(rename="record_relation", rename_all="lowercase")]
pub enum RecordRelation {
    Parent,
    Child
}

pub trait Test {

}
