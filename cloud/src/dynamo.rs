pub mod types;

use std::collections::hash_map::{HashMap, RandomState};
use super::auth::types::CgUser;
use rusoto_core::Region;
use dynomite::{
    retry::{Retries, Policy}, dynamodb::{DynamoDbClient, DynamoDb, KeySchemaElement,
        PutRequest, DeleteItemInput, QueryInput, ExpectedAttributeValue, 
        AttributeDefinition,  GetItemInput, WriteRequest, PutItemInput, DeleteRequest, 
        UpdateItemInput, ListTablesInput, CreateTableInput, AttributeValue, 
    },
    Attribute, Item, FromAttributes, DynamoDbExt, attr_map,
};

#[derive(Clone)]
pub struct DynamoClient {
    db: DynamoDbClient,
}

impl DynamoClient { 

    pub fn new() -> Result<Self, String> {
        let region = Region::UsWest2;
        let _retry_policy = Policy::default();
        Ok(Self { db: DynamoDbClient::new(region) })

    }

    pub async fn insert_user(&self, cuser: CgUser) -> Result<(), String> {
        let mut user: HashMap<String, AttributeValue> = HashMap::new();
        let mut attr: HashMap<String, AttributeValue> = HashMap::new();
        attr.insert("enabled".into(), AttributeValue { 
                bool: Some(cuser.enabled), ..Default::default() 
            });
        user.insert("attr".into(), AttributeValue { 
                m: Some(attr),  ..Default::default()  
            }).unwrap();
        user.insert("username".into(), AttributeValue { 
                s: Some(cuser.username), ..Default::default() 
            });
        match self.db.put_item(PutItemInput {
            table_name: "diuser".into(),
            item: user, ..Default::default()
        }).await {
           Ok(res) => Ok(()),
           Err(e) => Err(e.to_string())
        }
    }

    pub async fn create_table(&self, 
        table: &str, 
        attrs_types: Vec<(&str, &str, bool)>, // name, type, is_key
    ) -> Result<(), String> 
    {
        let (attrs, keys): (Vec<AttributeDefinition>, Vec<KeySchemaElement>) 
            = attrs_types.into_iter().fold(
                (Vec::<AttributeDefinition>::new(), Vec::<KeySchemaElement>::new()), 
                |(mut attrs, mut keys), att| 
            {
                attrs.push(AttributeDefinition {
                    attribute_name: att.0.into(),
                    attribute_type: att.1.into(), ..Default::default()
                });
                if att.2 {
                    keys.push(KeySchemaElement { //TODO deal with range key type
                        attribute_name: att.0.into(),
                        key_type: "HASH".into(), ..Default::default()
                    });
                }
                (attrs, keys)
            });
        match self.db.create_table(CreateTableInput {
            table_name: table.to_string(),
            attribute_definitions: attrs,
            key_schema: keys, ..Default::default()
        }).await {
            Ok(_resp) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn get_user_by_sub(&self, sub: String) -> Result<CgUser, String> {
        let mut key: HashMap<String, AttributeValue> = HashMap::new();
        key.insert("uid".to_string(), AttributeValue {
            s: Some(sub), ..Default::default()
        });
        let req = GetItemInput {
            table_name: "diuser".to_string(),
            key, ..Default::default()
        };
        match self.db.get_item(req).await {
            Ok(res) => Ok(CgUser::default()),
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


