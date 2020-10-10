use serde_json::json;
use rusoto_core::Region;
use rusoto_s3::{S3Client, S3, Bucket, ListBucketsOutput};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

#[tokio::main]
pub async fn main() {
    connect_dynamodb().await;
    connect_s3().await;
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

pub struct S3Client {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
