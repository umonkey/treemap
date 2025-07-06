use super::file_storage_interface::FileStorageInterface;
use crate::config::{AwsConfig, Secrets};
use crate::services::{Locatable, Locator};
use crate::types::*;
use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::ObjectCannedAcl;
use aws_sdk_s3::Client;
use log::{debug, error, info};
use std::sync::Arc;

pub struct S3FileStorage {
    client: Client,
    bucket: String,
}

impl S3FileStorage {
    pub async fn new(secrets: Arc<Secrets>) -> Result<Self> {
        let s3_bucket = secrets.require("FILES_BUCKET")?;

        let s3_config = AwsConfig::files(secrets)?;
        let client = Client::new(&s3_config);

        info!("S3 client initialized.");

        Ok(Self {
            client,
            bucket: s3_bucket.to_string(),
        })
    }
}

impl Locatable for S3FileStorage {
    fn create(locator: &Locator) -> Result<Self> {
        let secrets = locator.get::<Secrets>()?;
        let svc = futures::executor::block_on(Self::new(secrets))?;
        Ok(svc)
    }
}

#[async_trait]
impl FileStorageInterface for S3FileStorage {
    async fn read_file(&self, id: u64) -> Result<Vec<u8>> {
        debug!("Reading file {} from S3.", id);

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
                        error!("Error reading file: {e}");
                        Err(Error::FileDownload)
                    }
                }
            }

            Err(e) => {
                error!("Error downloading file: {e}");
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
            error!("Error uploading file {id} to S3: {e:?}");
            return Err(Error::FileUpload);
        }

        info!("File {} written to S3, length={}", id, bytes.len());

        Ok(())
    }
}
