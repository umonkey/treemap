use super::aws_config::AwsConfig;
use super::base::{CompletedPart, FileStorageInterface};
use crate::infra::config::Config;
use crate::infra::secrets::Secrets;
use crate::types::*;
use async_trait::async_trait;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::{
    CompletedMultipartUpload, CompletedPart as S3CompletedPart, ObjectCannedAcl,
};
use aws_sdk_s3::Client;
use log::{debug, error, info};
use std::time::{Duration, Instant};

pub struct S3FileStorage {
    client: Client,
    bucket: String,
}

impl S3FileStorage {
    pub fn new(config: &Config, secrets: &Secrets) -> Result<Self> {
        let s3_bucket = config.files_bucket.clone().ok_or(Error::Config(
            "FILES_BUCKET config option not set".to_string(),
        ))?;

        let s3_region = config.files_region.clone().ok_or(Error::Config(
            "FILES_REGION config option not set".to_string(),
        ))?;

        let s3_endpoint = config.files_endpoint.clone().ok_or(Error::Config(
            "FILES_ENDPOINT config option not set".to_string(),
        ))?;

        let s3_key = secrets
            .files_key
            .clone()
            .ok_or(Error::Config("FILES_KEY secret not set".to_string()))?;

        let s3_secret = secrets
            .files_secret
            .clone()
            .ok_or(Error::Config("FILES_SECRET secret not set".to_string()))?;

        let s3_config = AwsConfig::files(&s3_key, &s3_secret, &s3_region, &s3_endpoint)?;

        let client = Client::new(&s3_config);

        info!("S3 file storage client initialized.");

        Ok(Self {
            client,
            bucket: s3_bucket.to_string(),
        })
    }
}

#[async_trait]
impl FileStorageInterface for S3FileStorage {
    async fn read_file(&self, id: u64) -> Result<Vec<u8>> {
        debug!("Reading file {} from S3.", id);
        let start = Instant::now();

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
                        info!(
                            "File {} read, {} bytes in {}ms.",
                            id,
                            body.len(),
                            start.elapsed().as_millis()
                        );
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
        let start = Instant::now();

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
            error!("Error uploading file {} to S3: {:?}", id, e);
            return Err(Error::FileUpload);
        }

        info!(
            "File {} written to S3, length={} in {}ms",
            id,
            bytes.len(),
            start.elapsed().as_millis()
        );

        Ok(())
    }

    async fn create_upload_url(&self, id: u64) -> Result<String> {
        let key = id.to_string();
        let expires_in = Duration::from_secs(24 * 3600);
        let config = PresigningConfig::builder()
            .expires_in(expires_in)
            .build()
            .map_err(|e| Error::Config(e.to_string()))?;

        let presigned_request = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(config)
            .await
            .map_err(|e| {
                error!("Error creating presigned URL: {e:?}");
                Error::FileUpload
            })?;

        Ok(presigned_request.uri().to_string())
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        let res = self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await;

        match res {
            Ok(_) => Ok(true),
            Err(e) => {
                let service_error = e.into_service_error();
                if service_error.is_not_found() {
                    Ok(false)
                } else {
                    error!("Error checking file existence: {service_error:?}");
                    Err(Error::FileDownload)
                }
            }
        }
    }

    async fn start_multipart_upload(&self, key: &str) -> Result<String> {
        let res = self
            .client
            .create_multipart_upload()
            .bucket(&self.bucket)
            .key(key)
            .acl(ObjectCannedAcl::PublicRead)
            .content_type("video/mp4")
            .send()
            .await
            .map_err(|e| {
                error!("Error starting multipart upload: {e:?}");
                Error::FileUpload
            })?;

        Ok(res.upload_id().ok_or(Error::FileUpload)?.to_string())
    }

    async fn create_upload_part_url(
        &self,
        key: &str,
        upload_id: &str,
        part_number: i32,
    ) -> Result<String> {
        let expires_in = Duration::from_secs(24 * 3600);
        let config = PresigningConfig::builder()
            .expires_in(expires_in)
            .build()
            .map_err(|e| Error::Config(e.to_string()))?;

        let presigned_request = self
            .client
            .upload_part()
            .bucket(&self.bucket)
            .key(key)
            .upload_id(upload_id)
            .part_number(part_number)
            .presigned(config)
            .await
            .map_err(|e| {
                error!("Error creating presigned part URL: {e:?}");
                Error::FileUpload
            })?;

        Ok(presigned_request.uri().to_string())
    }

    async fn complete_multipart_upload(
        &self,
        key: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<()> {
        let completed_parts: Vec<S3CompletedPart> = parts
            .into_iter()
            .map(|p| {
                S3CompletedPart::builder()
                    .part_number(p.part_number)
                    .e_tag(p.etag)
                    .build()
            })
            .collect();

        let multipart_upload = CompletedMultipartUpload::builder()
            .set_parts(Some(completed_parts))
            .build();

        self.client
            .complete_multipart_upload()
            .bucket(&self.bucket)
            .key(key)
            .upload_id(upload_id)
            .multipart_upload(multipart_upload)
            .send()
            .await
            .map_err(|e| {
                error!("Error completing multipart upload: {e:?}");
                Error::FileUpload
            })?;

        Ok(())
    }
}
