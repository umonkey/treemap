use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use aws_sdk_s3::presigning::PresigningConfig;
use log::{debug, error};
use std::time::Duration;

use crate::Result;
use crate::errors::Error;
use crate::utils::get_s3_bucket;

const UPLOAD_TTL: u64 = 3600;

pub struct S3Service {
    client: Client,
    bucket: String,
}

impl S3Service {
    pub async fn init() -> Result<Self> {
        let bucket = get_s3_bucket()?;

        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;

        let client = Client::new(&config);

        Ok(Self {
            client,
            bucket,
        })
    }

    pub async fn get_upload_url(&self, key: &str) -> Result<String> {
        let expires_in = Duration::from_secs(UPLOAD_TTL);

        let expiration = match PresigningConfig::expires_in(expires_in) {
            Ok(expiration) => expiration,

            Err(e) => {
                error!("Error creating expiration: {}", e);
                return Err(Error::FileUpload);
            },
        };

        let req = self.client
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
            },

            Err(e) => {
                error!("Error creating presigned URL: {}", e);
                Err(Error::FileUpload)
            },
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
        env::set_var("AWS_REGION", "moon");
        env::set_var("TREEMAP_S3_BUCKET", "tree-files");

        if let Err(_) = env_logger::try_init() {
            debug!("env_logger already initialized.");
        };

        S3Service::init().await
    }

    #[tokio::test]
    async fn test_get_upload_url() -> Result<()> {
        let service = setup().await?;
        let url = service.get_upload_url("sample.txt").await?;

        assert!(url.starts_with("https://tree-files.s3.moon.amazonaws.com/sample.txt?"));
        assert!(url.contains("x-id=PutObject"));
        assert!(url.contains("X-Amz-SignedHeaders=host"));
        assert!(url.contains("X-Amz-Signature="));

        Ok(())
    }
}
