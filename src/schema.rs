use chrono::{DateTime, Utc};
use uuid::Uuid;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use juniper::{FieldResult, RootNode};
use juniper::Object;

#[derive(GraphQLEnum)]
pub enum Status  {
    Active,
    Archived,
    Completed,
    Deleted,
    Paused,
}

#[derive(GraphQLEnum)]
pub enum Visibility {
    Private,
    InviteOnly,
    MutualsOnly,
    Public,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    pub id: Uuid,
    pub uid: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub visibility: Visibility,
    pub status: Status,
    pub attributes: Vec<String>,
    pub notes: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/*
pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {

    pub async fn users(db: &div_db::Db) -> Vec<User> {
        div_db::models::User::get_all(&db.pool).await.unwrap()
    }
}

*/
