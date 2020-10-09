use sqlx::{FromRow, types::chrono::{DateTime, Utc}, postgres::PgRow, prelude::*};
use serde::{Serialize, Deserialize};
use crate::{
    db::Db, 
    models::{ Group, Model,
        Record, UserInfo, Item, link::{Link, LinkedTo},
    },
};
use sqlx::Postgres;
use async_trait::async_trait;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLogin { pub username: String, pub password: String }

#[derive(Serialize, Deserialize, Clone)]
pub struct UserRegister { email: String, username: String, password: String }

#[serde(rename_all="camelCase")]
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub password: String,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl User {

    pub fn new<T, U, V>(email: T, username: U, password: V,) -> User
        where T: Into<String>, U: Into<String>, V: Into<String> {
        User { 
            email: email.into(), 
            username: username.into(), 
            password: password.into(),
            ..User::default()
        }
    }

    pub async fn insert(self, db: &Db) -> sqlx::Result<Self> {
        println!("INSERTING {} {} {}", &self.username, &self.email, &self.password);
        let mut conn = db.pool.acquire().await?;
        let res: i32 = sqlx::query
            ("INSERT INTO Users (email, username, password, created_at)
              VALUES ($1, $2, $3, $4) RETURNING id") 
            .bind(&self.email)
            .bind(&self.username)
            .bind(&self.password)
            .bind(&self.created_at)
            .fetch_one(&mut conn).await?
            .get("id");
        conn.release();
        UserInfo::from(self.clone()).insert(db).await?;
        Ok( Self { id: Some(res), ..self } )
    }

    pub async fn delete_by_username(db: &Db, username: String) -> sqlx::Result<i32> {
        let res: i32 = sqlx::query_scalar
            ("DELETE FROM Users WHERE username=$1 RETURNING id")
            .bind(username)
            .fetch_one(&db.pool).await?;
        Ok(res as i32)
    }

    pub async fn delete_by_id(db: &Db, id: i32) -> sqlx::Result<Option<i32>> {
        let res: Option<i32> = sqlx::query_scalar
            ("DELETE FROM Users WHERE id=$1 RETURNING id")
            .bind(id)
            .fetch_optional(&db.pool).await?;
        Ok(res)
    }

    pub async fn get_all(db: &Db) -> sqlx::Result<Vec<User>> {
        let res: Vec<User> = sqlx::query_as::<Postgres, User>
            ("SELECT * FROM Users;") 
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }

    pub async fn get_by_id(db: &Db, id: i32) -> sqlx::Result<Option<User>> {
        let res: Option<User> = sqlx::query_as::<Postgres, User>
            ("SELECT * FROM Users WHERE id=$1") 
            .bind(id)
            .fetch_optional(&db.pool)
            .await?;
        Ok(res)
    }

    /// Get a user by username
    pub async fn get_by_username(db: &Db, username: String) -> sqlx::Result<Option<User>> {
        let res: Option<User> = sqlx::query_as::<Postgres, User>
            ("SELECT * FROM Users WHERE username=$1") 
            .bind(username)
            .fetch_optional(&db.pool)
            .await?;
        Ok(res)
    }

    // Get all records created by user
    pub async fn get_all_records(db: &Db, id: i32) -> sqlx::Result<Vec<Record>> {
        let res: Vec<Record> = sqlx::query_as::<Postgres, Record>
            ("SELECT * FROM Records r WHERE r.uid = $1")
            .bind(id)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    pub async fn get_all_items(db: &Db, id: i32) -> sqlx::Result<Vec<Item>> {
        let res: Vec<Item> = sqlx::query_as::<Postgres, Item>
            ("SELECT * FROM Items i WHERE i.uid = $1")
            .bind(id)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    pub async fn get_linked_records(db: &Db, id: i32) -> sqlx::Result<Vec<Record>> {
        let res = sqlx::query_as::<Postgres, Record>
            ("SELECT r.id, r.name, r.status, r.visibility, r.created_at
              FROM Records r INNER JOIN UserRecordLinks ur ON r.id = ur.rid
                   AND ur.uid = $1")
            .bind(id)
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }

    pub async fn get_linked_items(db: &Db, id: i32) -> sqlx::Result<Vec<Record>> {
        let res = sqlx::query_as::<Postgres, Record>
            ("SELECT i.id, i.name, i.status, i.visibility, i.created_at
              FROM Items i INNER JOIN UserItemLinks ui ON ui.iid = i.id
                   AND ui.uid = $1")
            .bind(id)
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }

    pub async fn add_new_record(db: &Db, uid: i32, rec_name: String,
        ) -> sqlx::Result<Record> 
    {
        let rec: Record = Record::new(uid, rec_name).insert(db).await?;
        Link::new(Some(uid), rec.id).insert::<User, Record>(db).await?;
        Ok(rec)
    }

    pub async fn add_existing_record(db: &Db, uid: i32, record: Record) 
        -> sqlx::Result<Option<Record>> {
        if uid == record.id.unwrap() { return Ok(None); }
        let res = Link::new(Some(uid), record.id).insert::<User, Record>(db).await?;
        Ok(Some(record))
    }

    pub async fn add_new_item(
        db: &Db, uid: i32, item_name: String,
    ) -> sqlx::Result<Item> {
        let item = Item::new(uid, item_name).insert(db).await?;
        Link::new(Some(uid), item.id).insert::<User, Item>(db).await?;
        Ok(item)
    }

    pub async fn add_existing_item(db: &Db, uid: i32, item: Item) 
        -> sqlx::Result<Option<Item>> 
    {
        if uid == item.id.unwrap() { return Ok(None); }
        let link = Link::new(Some(uid), item.id).insert::<User, Item>(db).await?;
        Ok(Some(item))
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            id: None,
            username: String::from(""),
            email: String::from(""),
            password: String::from(""),
            created_at: Utc::now(),
        }   
    }
}

impl From<&'static PgRow> for User {
    fn from(row: &'static PgRow) -> Self {
        User::from_row(row).unwrap()
    }
}

#[async_trait::async_trait]
impl Model for User {
    fn table() -> String { String::from("Users") }
    fn foreign_id() -> String { String::from("uid") }
    fn fields() ->  Vec<String> { 
        let fields = vec!["id", "uid", "username", "password", "email", "created_at"];
        fields.into_iter()
            .map(|field| field.to_string())
            .collect::<Vec<String>>()
    }
    fn id(self) -> i32 { self.id.expect("ID not set for Item") }
    
}

impl LinkedTo<Record> for User {}
impl LinkedTo<Group> for User {}
impl LinkedTo<Item> for User {}
