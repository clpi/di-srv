pub mod types;

use rusoto_core::{Region, ByteStream};
use tokio::io::AsyncRead;
use rusoto_s3::{
    S3, S3Client, DeleteObjectRequest, PutObjectRequest, GetObjectRequest,
    JSONInput, JSONTypeSerializer, CSVInput, S3Error, Object, Rule, Bucket,
    ListObjectsV2Request, CreateBucketRequest, DeleteBucketRequest,
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
        Self { s3: client }
    }

    pub async fn create_bucket(&self, name: &str) -> Result<String, String> {
        match self.s3.create_bucket(CreateBucketRequest {
            bucket: name.into(), ..Default::default()
        }).await {
            Ok(bucket) => { Ok(bucket.location.unwrap()) },
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn delete_bucket(&self, name: &str) -> Result<(), String> {
        match self.s3.delete_bucket(DeleteBucketRequest {
            bucket: name.into(), ..Default::default()
        }).await {
            Ok(_) => { Ok(()) },
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn get_object(&self, bucket: &str, key: &str) -> Result<ByteStream, String> {
        match self.s3.get_object(GetObjectRequest {
            bucket: bucket.into(),
            key: key.into(), ..Default::default()
        }).await {
            Ok(obj) => Ok(obj.body.unwrap()),
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn list_buckets(&self) -> Result<Vec<Bucket>, String> {
        match self.s3.list_buckets().await {
            Ok(resp) => match resp.buckets {
                Some(buckets) => Ok(buckets),
                None => Err("No buckets".to_string()),
            },
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn list_objects(&self, bucket: &str, path: Option<String>)
        -> Result<Vec<Object>, String> 
    {
        match self.s3.list_objects_v2(ListObjectsV2Request {
            bucket: bucket.into(),
            prefix: path, ..Default::default()  //TODO should handle continuation token
        }).await {
            Ok(resp) => match resp.contents {
                Some(contents) => Ok(contents),
                None => Err("No objects in bucket + prefix".into()),
            },
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn put(&self) -> () {}

    pub async fn get(&self) -> () {}

    pub async fn delete(&self) -> () {}
}
