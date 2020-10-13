use rusoto_core::Region;
use rusoto_s3::{
    S3, S3Client, DeleteObjectRequest, PutObjectRequest, GetObjectRequest,
    JSONInput, JSONTypeSerializer, CSVInput, S3Error, Object, Rule, Bucket,
    ListObjectsRequest, 
};
use std::io::Read;

#[derive(Clone)]
pub struct Client {
    s3: S3Client,
}

impl Client {

    pub fn new() -> Client {
        let region = Region::UsWest2;
        let client = S3Client::new(region);

        /*
        Client {
            region: region.to_owned(),
            s3: S3Client::new(region),
            bucket_name: std::env::var("AWS_S3_BUCKET_NAME").unwrap(),
        }
        */
        Self { s3: client }
    }

    pub async fn put() -> () {}

    pub async fn get() -> () {}

    pub async fn delete() -> () {}
}
