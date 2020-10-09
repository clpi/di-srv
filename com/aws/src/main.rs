use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

#[async_std::main]
pub async fn main() {
    connect_dynamodb().await;
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
    let t = 3;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
