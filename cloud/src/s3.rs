pub mod types;

use std::fs::File;
use rusoto_core::{Region, ByteStream};
use tokio::io::{AsyncRead, BufReader, AsyncReadExt};
use futures::{FutureExt, TryStreamExt};
use rusoto_s3::{
    S3, S3Client, DeleteObjectRequest, PutObjectRequest, GetObjectRequest,
    JSONInput, JSONTypeSerializer, CSVInput, S3Error, Object, Rule, Bucket,
    ListObjectsV2Request, CreateBucketRequest, DeleteBucketRequest,
};
use std::{io::Read, collections::HashMap};

#[derive(Clone)]
pub struct Client {
    s3: S3Client,
    bucket: Option<&'static str>, // Take this out?
}

impl Client {

    pub fn new() -> Client {
        let region = Region::UsWest2;
        let client = S3Client::new(region);
        Self { s3: client, bucket: None }
    }

    pub fn with_bucket(bucket: &'static str) -> Self {
        let region = Region::UsWest2;
        let client = S3Client::new(region);
        Self { s3: client, bucket: Some(bucket.into()) }
    }

    pub async fn create_bucket(&self, name: &str) -> Result<String, String> {
        match self.s3.create_bucket(CreateBucketRequest {
            bucket: name.into(), ..Default::default()
        }).await {
            Ok(bucket) => { Ok(bucket.location.unwrap()) },
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn put_object(&self, local_path: &str, key: &str) -> String {
        let mut buf: Vec<u8> = Vec::new();
        File::open(local_path).unwrap()
            .read_to_end(&mut buf);
        let put_request = PutObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: key.to_owned(),
            body: Some(buf.into()),
            ..Default::default()
        };
        let _res = self
            .s3
            .put_object(put_request)
            .await
            .expect("Failed to put test object");

        self.url(key)
    }

    pub async fn delete_bucket(&self, name: &str) -> Result<(), String> {
        match self.s3.delete_bucket(DeleteBucketRequest {
            bucket: name.into(), ..Default::default()
        }).await {
            Ok(_) => { Ok(()) },
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn get_object(&self, bucket: &str, key: &str) 
        -> tokio::io::Result<Vec<u8>> 
    {
        match self.s3.get_object(GetObjectRequest {
            bucket: bucket.into(),
            key: key.into(), ..Default::default()
        }).await {
            Ok(obj) => match obj.body {
                Some(content) => {
                    let mut buf: Vec<u8> = Vec::new();
                    content.into_async_read().read_to_end(&mut buf).await?;
                    Ok(buf)
                },
                None => Err(tokio::io::ErrorKind::WriteZero.into())
            },
            Err(_err) => Err(tokio::io::ErrorKind::NotFound.into())
        }
    }

    pub async fn delete(&self, bucket: &str, key: &str) -> Result<String, String> {
        match self.s3.delete_object(DeleteObjectRequest {
            bucket: bucket.into(),
            key: key.into(), ..Default::default()
        }).await {
            Ok(_obj) => Ok("Deleted object".into()),
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

    pub async fn put_item(
        &self, bucket: &str, 
        path: &str, 
        object: Vec<u8>,
        metadata: Option<HashMap<String, String>>
        ) -> Result<String, String> 
    {
        match self.s3.put_object(PutObjectRequest {
            bucket: bucket.into(),
            body: Some(object.into()),
            key: path.into(),
            metadata, ..Default::default()
        }).await {
            Ok(resp) => Ok(resp.e_tag.unwrap()),
            Err(err) => Err(err.to_string())
        }
    }

    pub async fn get(&self) -> () {}

    pub async fn delete(&self) -> () {}
}

/// Represents an S3 client concerned with a single bucket in a region
#[derive(Clone)]
pub struct S3Bucket {
    client: S3Client,
    bucket: String,
}

impl S3Bucket {

    pub fn new(bucket: &str) -> Self { 
        let region = Region::UsWest2;
        Self {
            client: S3Client::new(region),
            bucket: bucket.into(),
        }
    }
    
    pub async fn get_object(key: &str) -> Result<Object, String> {
        Ok(Object::default())
    }
}
