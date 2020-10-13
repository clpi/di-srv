use std::collections::HashMap;
use super::auth::types::CgUser;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, Get, Put, Delete, Update,
    GetItemInput, WriteRequest, PutItemInput, DeleteRequest, UpdateItemInput,
    ListTablesInput, CreateTableInput, AttributeValue,
};

pub struct DynamoClient {
    db: DynamoDbClient,
}

impl DynamoClient { 
    pub fn new() -> Self {
        let region = Region::UsWest2;
        Self { db: DynamoDbClient::new(region) }

    }

    pub async fn create_user(&self, cuser: CgUser) -> Result<(), String> {
        let mut user: HashMap<String, AttributeValue> = HashMap::new();
        user.insert("uid".into(), AttributeValue { s: Some(cuser.username), ..Default::default() });
        match self.db.put_item(PutItemInput {
            table_name: "diuser".into(),
            item: user, ..Default::default()
        }).await {
           Ok(res) => Ok(()),
           Err(e) => Err(e.to_string())
        }
    }

    pub async fn list_tables(&self, user: CgUser) -> Result<Vec<String>, String> {
        match self.db.list_tables(ListTablesInput { ..Default::default() }).await{
            Ok(res) => Ok(res.table_names.unwrap()),
            Err(e) => Err(e.to_string())
                
        }
    }

    pub async fn write_user(&self, user: CgUser) -> Result<(), String> {
        Ok(())
    }
}

