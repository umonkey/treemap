use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::ObjectCannedAcl;
use aws_sdk_s3::Client;
use log::{debug, error, info};
use std::time::Duration;

use crate::config::S3Config;
use crate::types::{Error, Result};

const UPLOAD_TTL: u64 = 3600;

pub struct S3Service {
    client: Option<Client>,
    bucket: String,
}

impl S3Service {
    pub async fn new() -> Result<Self> {
        let config = match S3Config::from_env() {
            Ok(value) => value,

            Err(Error::EnvNotSet) => {
                return Ok(Self {
                    client: None,
                    bucket: "".to_string(),
                });
            }

            Err(e) => return Err(e),
        };

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
            client: Some(client),
            bucket: config.bucket.to_string(),
        })
    }

    pub fn is_enabled(&self) -> bool {
        self.client.is_some()
    }

    pub async fn get_upload_url(&self, key: &str) -> Result<String> {
        let expires_in = Duration::from_secs(UPLOAD_TTL);

        let expiration = match PresigningConfig::expires_in(expires_in) {
            Ok(expiration) => expiration,

            Err(e) => {
                error!("Error creating expiration: {}", e);
                return Err(Error::FileUpload);
            }
        };

        let req = self
            .get_client()?
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(expiration)
            .await;

        match req {
            Ok(req) => {
                let url = req.uri().to_string();
                debug!("Presigned URL: {}", url);
                Ok(url)
            }

            Err(e) => {
                error!("Error creating presigned URL: {}", e);
                Err(Error::FileUpload)
            }
        }
    }

    /**
     * Uploads all local files older than 1 hour to S3.
     */
    pub async fn upload_all(&self) -> Result<()> {
        for entry in glob::glob("var/files/*").expect("Error listing files.") {
            match entry {
                Ok(path) => {
                    let name = path
                        .file_name()
                        .expect("Error extracting file name.")
                        .to_str()
                        .expect("Error converting file name to string.");
                    let id = name.parse::<u64>().expect("Error parsing file name.");
                    self.upload_file(id).await.expect("Error uploading file.");
                }

                Err(e) => {
                    error!("Error listing files: {}", e);
                }
            }
        }

        Ok(())
    }

    pub async fn upload_file(&self, id: u64) -> Result<()> {
        let local_path = format!("var/files/{}", id);

        if !std::path::Path::new(&local_path).exists() {
            error!("File not found, cannot upload: {}", id);
            return Err(Error::FileNotFound);
        }

        let body = match ByteStream::from_path(local_path.clone()).await {
            Ok(value) => value,

            Err(e) => {
                error!("Error reading file: {}", e);
                return Err(Error::FileUpload);
            }
        };

        let res = self
            .get_client()?
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

        if let Err(e) = std::fs::remove_file(local_path) {
            error!("Error deleting file {}: {}", id, e);
            return Err(Error::FileUpload);
        }

        info!("File {} moved to S3.", id);
        Ok(())
    }

    fn get_client(&self) -> Result<&Client> {
        match &self.client {
            Some(client) => Ok(client),
            None => Err(Error::S3Disabled),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    use std::env;

    async fn setup() -> Result<S3Service> {
        env::set_var("AWS_ACCESS_KEY_ID", "test");
        env::set_var("AWS_SECRET_ACCESS_KEY", "secret");
        env::set_var("S3_BUCKET", "tree-files");
        env::set_var("S3_REGION", "moon");
        env::set_var("S3_ENDPOINT", "https://moon.digitaloceanspaces.com");

        if let Err(_) = env_logger::try_init() {
            debug!("env_logger already initialized.");
        };

        S3Service::new().await
    }

    #[tokio::test]
    async fn test_get_upload_url() -> Result<()> {
        let service = setup().await?;
        let url = service.get_upload_url("sample.txt").await?;

        assert!(url.starts_with("https://tree-files.moon.digitaloceanspaces.com/sample.txt?"));
        assert!(url.contains("x-id=PutObject"));
        assert!(url.contains("X-Amz-SignedHeaders=host"));
        assert!(url.contains("X-Amz-Signature="));

        Ok(())
    }
}
