use crate::models::{
    Group, User, Record, Item, Model,
    link::{LinkType, Link},
};

// TODO No separate "Link" and "Relation" tables -- unique trio of id1, id2, and relation
// Relationships are enumerated and stored as strings, not as a separate table for each
// linkage. Some will be very few: UserUser -> (Friend, Blocked, Muted)
//
// Relations are then vectors: Say a record holds the relationships with a item:
// TODO A specific user-made relation should be separated from an enumerated
// relation necessary for backend functioning -- could be specified with a separate
// field which is nullable which contains a foreign key to a user-made relations table
//
// TODO All of this might be waaaay better implemented as a graph.......

pub enum Relation {
    HasA,
    Created,
}

pub enum GroupUserRelation {
    MemberOf,
    AdminOf,
    ModeratorOf,
}

pub enum GroupGroupRelation {
    Partnered,
    Associated,
    Blocked,
    None,
}

pub enum UserUserRelation {
    Mutual,
    Following,
    Blocked,
    Muted,
    None,
}

pub enum GroupRelation {
    Group,
    User,
    Record
}

pub enum UserUserRelations {
    Friend,
    Blocked,
    Muted,
}

pub enum GroupUserRelations {
    Admin,
    Member,
    Moderator,
}

pub enum UserRecordRelations {
    Admin,
    Member,
    Moderator,
    CreatedBy,
}


