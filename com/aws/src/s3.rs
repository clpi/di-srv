use rusoto_core::Region;
use rusoto_s3::S3;
use rusoto_s3::{DeleteObjectRequest, PutObjectRequest, S3Client};
use std::io::Read;

pub struct Client {

    
}

impl Client {

    pub fn new() -> Client {
        let region = Region::default();

        Client {
            region: region.to_owned(),
            s3: S3Client::new(region),
            bucket_name: std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
        }
    }

    pub async fn put() -> () {}

    pub async fn get() -> () {}

    pub async fn delete() -> () {}
}
