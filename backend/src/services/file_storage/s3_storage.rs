use async_trait::async_trait;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::ObjectCannedAcl;
use aws_sdk_s3::Client;
use log::{error, info};

use crate::config::S3Config;
use crate::services::FileStorageInterface;
use crate::types::{Error, Result};

pub struct S3FileStorage {
    client: Client,
    bucket: String,
}

impl S3FileStorage {
    pub async fn new() -> Result<Self> {
        let config = S3Config::from_env()?;

        let s3_config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(config.region.to_string()))
            .endpoint_url(config.endpoint.to_string())
            .load()
            .await;

        let client = Client::new(&s3_config);

        info!(
            "S3 client initialized, bucket={} region={} endpoint={}.",
            config.bucket.to_string(),
            config.region.to_string(),
            config.endpoint.to_string()
        );

        Ok(Self {
            client,
            bucket: config.bucket.to_string(),
        })
    }
}

#[async_trait]
impl FileStorageInterface for S3FileStorage {
    async fn read_file(&self, id: u64) -> Result<Vec<u8>> {
        let res = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(id.to_string())
            .send()
            .await;

        match res {
            Ok(res) => {
                let body = res.body.collect().await;
                match body {
                    Ok(body) => {
                        let body = body.into_bytes();
                        info!("File {} read, {} bytes.", id, body.len());
                        return Ok(body.to_vec());
                    }

                    Err(e) => {
                        error!("Error reading file: {}", e);
                        Err(Error::FileDownload)
                    }
                }
            }

            Err(e) => {
                error!("Error downloading file: {}", e);
                Err(Error::FileDownload)
            }
        }
    }

    async fn write_file(&self, id: u64, bytes: &[u8]) -> Result<()> {
        let body = ByteStream::from(bytes.to_vec());

        let res = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(id.to_string())
            .body(body)
            .acl(ObjectCannedAcl::PublicRead)
            .content_type("image/jpeg")
            .send()
            .await;

        if let Err(e) = res {
            error!("Error uploading file {} to S3: {}", id, e);
            return Err(Error::FileUpload);
        }

        info!("File {} written.", id);

        Ok(())
    }

    async fn find_files(&self) -> Result<Vec<u64>> {
        Ok(Vec::new())
    }
}
