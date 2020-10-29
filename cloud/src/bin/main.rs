use serde_json::json;
use rusoto_core::Region;
use rusoto_s3::{S3Client, S3, Bucket, ListBucketsOutput};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
use div_cloud::cognito::*;

#[tokio::main]
pub async fn main() {
    let client = CognitoClient::new();
    let pools =  client.list_user_pools().await.unwrap();
    pools.iter().for_each(|pool| { println!("Pool: {:?}", pool.name) });
    //client.delete_user("new".to_string()).await.unwrap();
    let user = CgUserSignup { 
        username: "keewa".into(), 
        password: "Keewa123!".into(),
        email: "keewa@div.is".into(),
    };
    client.signup_user(user).await.unwrap();
    client.confirm_signup("keewa".to_string()).await.unwrap();
    let user = client.get_user("testman".into()).await.unwrap();
    let user2 = client.get_user("keewa".into()).await.unwrap();
    println!("User: {:?}", serde_json::to_string(&user).unwrap());
    println!("User: {:?}", serde_json::to_string(&user2).unwrap());
}

pub async fn connect_dynamodb() {
    let client = DynamoDbClient::new(Region::UsWest2);
    let list_tables: ListTablesInput = Default::default();
    match client.list_tables(list_tables).await {
        Ok(output) => match output.table_names {
            Some(table_names) => {
                println!("Tables: ");
                for table in table_names.iter() {
                    println!("Tables: {:?}", table);
                }
            },
            None => { println!("No tables in DB"); },
        },
        Err(e) => { println!("Error {:?}", e) }
    };
}

pub async fn connect_s3() {
    let client = S3Client::new(Region::UsWest2);
    let buckets = client.list_buckets().await;
    match buckets {
        Ok(buc) => {
            println!("Buckets:");
            for bucket in buc.buckets {
                println!("{:?}", bucket);
            }
        },
        Err(_) => { println!("No buckets in region") }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
