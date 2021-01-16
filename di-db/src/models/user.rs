use sqlx::{FromRow,
    types::{
        chrono::{DateTime, Utc},
        uuid::{Uuid, Variant}
    },
    postgres::PgRow, prelude::*
};
use serde::{Serialize, Deserialize};
use dynomite::{Item as DItem, FromAttributes, Attribute, attr_map};
use crate::{
    db::Db,
    models::{
        Group, Model, Record, UserInfo, Item,
    },
};
use sqlx::Postgres;
use div_cloud::dynamo::DynamoClient;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserRegister {
    pub email: String,
    pub username: String,
    pub password: String
}

#[derive(DItem, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    #[dynomite(partition_key)]
    #[serde(default="Uuid::new_v4")]
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default="Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl User {

    pub fn new<T, U, V>(email: T, username: U, password: Option<String>) -> User
        where T: Into<String>, U: Into<String>, V: Into<String> {
        User {
            id: Uuid::new_v4(),
            email: email.into(),
            username: username.into(),
            password,
            ..User::default()
        }
    }

    pub async fn insert_db(self, db: &Db) -> sqlx::Result<Self> {
        let res: Uuid = sqlx::query
            ("INSERT INTO Users (email, username, password, created_at)
              VALUES ($1, $2, $3, $4) RETURNING id")
            .bind(&self.email)
            .bind(&self.username)
            .bind(&self.password)
            .bind(&self.created_at)
            .fetch_one(&db.pool).await?
            .get("id");
        let user_with_id = User { id: res, ..self };
        //UserInfo::from(user_with_id.clone()).insert(db).await?;
        Ok(user_with_id)
    }

    pub async fn insert_dynamo(self, db: &DynamoClient) -> Result<(), String> {
        db.insert("diuser".into(), self).await
    }

    pub async fn delete_by_username(db: &Db, username: String) -> sqlx::Result<Uuid> {
        let res: Uuid = sqlx::query_scalar
            ("DELETE FROM Users WHERE username=$1 RETURNING id")
            .bind(username)
            .fetch_one(&db.pool).await?;
        Ok(res as Uuid)
    }

    pub async fn delete_by_id(db: &Db, id: Uuid) -> sqlx::Result<Option<Uuid>> {
        let res: Option<Uuid> = sqlx::query_scalar
            ("DELETE FROM Users WHERE id=$1 RETURNING id")
            .bind(id)
            .fetch_optional(&db.pool).await?;
        Ok(res)
    }

    pub async fn get_all(db: &Db) -> sqlx::Result<Vec<User>> {
        let res: Vec<User> = sqlx::query_as::<Postgres, User>
            ("SELECT * FROM Users")
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }

    pub async fn get_by_id(db: &Db, id: Uuid) -> sqlx::Result<Option<User>> {
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
    pub async fn get_all_records(db: &Db, id: Uuid) -> sqlx::Result<Vec<Record>> {
        let res: Vec<Record> = sqlx::query_as::<Postgres, Record>
            ("SELECT * FROM Records r WHERE r.uid = $1")
            .bind(id)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    pub async fn get_item_by_name(
        db: &Db, uid: Uuid, item_name: String
        ) -> sqlx::Result<Option<Item>>
    {
        let res: Option<Item> = sqlx::query_as::<Postgres, Item>
            ("SELECT * FROM Items i WHERE i.uid = $1 AND i.name = $2")
            .bind(uid)
            .bind(item_name)
            .fetch_optional(&db.pool).await?;
        Ok(res)
    }

    pub async fn delete_item_by_name(
        db: &Db, uid: Uuid, item_name: String
        ) -> sqlx::Result<Uuid>
    {
        let res: Uuid = sqlx::query(
            "DELETE FROM Items i WHERE i.uid = $1 AND i.name = $2 RETURNING id")
            .bind(uid)
            .bind(item_name)
            .fetch_one(&db.pool).await?
            .get("id");
        Ok(res)
    }

    pub async fn get_all_items(db: &Db, id: Uuid) -> sqlx::Result<Vec<Item>> {
        let res: Vec<Item> = sqlx::query_as::<Postgres, Item>
            ("SELECT * FROM Items i WHERE i.uid = $1")
            .bind(id)
            .fetch_all(&db.pool).await?;
        Ok(res)
    }

    pub async fn get_linked_records(db: &Db, id: Uuid) -> sqlx::Result<Vec<Record>> {
        let res = sqlx::query_as::<Postgres, Record>
            ("SELECT r.id, r.name, r.status, r.visibility, r.created_at
              FROM Records r INNER JOIN UserRecordLinks ur ON r.id = ur.rid
                   AND ur.uid = $1")
            .bind(id)
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }

    pub async fn get_linked_items(db: &Db, id: Uuid) -> sqlx::Result<Vec<Record>> {
        let res = sqlx::query_as::<Postgres, Record>
            ("SELECT i.id, i.name, i.status, i.visibility, i.created_at
              FROM Items i INNER JOIN UserItemLinks ui ON ui.iid = i.id
                   AND ui.uid = $1")
            .bind(id)
            .fetch_all(&db.pool)
            .await?;
        Ok(res)
    }

    pub async fn get_named_record(db: &Db, uid: Uuid, rec_name: String,
        ) -> sqlx::Result<Record>
    {
        Ok(Record::default())
    }

    pub async fn get_named_item(db: &Db, uid: Uuid, item_name: String,
        ) -> sqlx::Result<Item>
    {
        Ok(Item::default())
    }

    pub async fn add_new_record(db: &Db, uid: Uuid, rec_name: String,
        ) -> sqlx::Result<Record>
    {
        let rec: Record = Record::new(uid, rec_name).insert(db).await?;
        Ok(rec)
    }

    pub async fn add_existing_record(db: &Db, uid: Uuid, record: Record)
        -> sqlx::Result<Option<Record>> {
        if uid == record.id { return Ok(None); }
        Ok(Some(record))
    }

    pub async fn add_new_item(
        db: &Db, uid: Uuid, item_name: String,
    ) -> sqlx::Result<Item> {
        let item = Item::new(uid, item_name);
        item.insert(db).await?;
        Ok(item)
    }

    pub async fn add_existing_item(db: &Db, uid: Uuid, item: Item)
        -> sqlx::Result<Option<Item>>
    {
        if uid == item.id { return Ok(None); }
        Ok(Some(item))
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            id: Uuid::new_v4(),
            username: String::from(""),
            email: String::from(""),
            password: None,
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
    fn id(self) -> Uuid { self.id }
}
