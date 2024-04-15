use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use log::{debug, error};
use std::time::Duration;

use crate::types::Error;
use crate::utils::{get_s3_bucket, get_s3_endpoint, get_s3_region};
use crate::Result;

const UPLOAD_TTL: u64 = 3600;

pub struct S3Service {
    client: Client,
    bucket: String,
}

impl S3Service {
    pub async fn init() -> Result<Self> {
        let bucket = get_s3_bucket()?;
        let region = get_s3_region()?;
        let endpoint = get_s3_endpoint()?;

        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(region))
            .endpoint_url(endpoint)
            .load()
            .await;

        let client = Client::new(&config);

        Ok(Self { client, bucket })
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
            .client
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    use std::env;

    async fn setup() -> Result<S3Service> {
        env::set_var("AWS_ACCESS_KEY_ID", "test");
        env::set_var("AWS_SECRET_ACCESS_KEY", "secret");
        env::set_var("TREEMAP_S3_BUCKET", "tree-files");
        env::set_var("TREEMAP_S3_REGION", "moon");
        env::set_var("TREEMAP_S3_ENDPOINT", "https://moon.digitaloceanspaces.com");

        if let Err(_) = env_logger::try_init() {
            debug!("env_logger already initialized.");
        };

        S3Service::init().await
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
