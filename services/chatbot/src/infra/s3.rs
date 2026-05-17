use crate::infra::config::Config;
use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_credential_types::Credentials;
use aws_sdk_s3::config::SharedCredentialsProvider;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::ObjectCannedAcl;
use aws_sdk_s3::Client;
use log::{error, info};

pub struct S3FileStorage {
    client: Client,
    bucket: String,
}

impl S3FileStorage {
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        let s3_bucket = config
            .files_bucket
            .clone()
            .ok_or_else(|| anyhow::anyhow!("FILES_BUCKET config option not set"))?;

        let s3_region = config
            .files_region
            .clone()
            .ok_or_else(|| anyhow::anyhow!("FILES_REGION config option not set"))?;

        let s3_endpoint = config
            .files_endpoint
            .clone()
            .ok_or_else(|| anyhow::anyhow!("FILES_ENDPOINT config option not set"))?;

        let s3_key = config
            .files_key
            .clone()
            .ok_or_else(|| anyhow::anyhow!("FILES_KEY config option not set"))?;

        let s3_secret = config
            .files_secret
            .clone()
            .ok_or_else(|| anyhow::anyhow!("FILES_SECRET config option not set"))?;

        let credentials = Credentials::new(s3_key, s3_secret, None, None, "chatbot");
        let credentials = SharedCredentialsProvider::new(credentials);

        let timeout = aws_config::timeout::TimeoutConfig::builder()
            .connect_timeout(std::time::Duration::from_secs(30))
            .build();

        let s3_config = SdkConfig::builder()
            .region(Region::new(s3_region))
            .credentials_provider(credentials)
            .timeout_config(timeout)
            .behavior_version(BehaviorVersion::latest())
            .endpoint_url(s3_endpoint)
            .build();

        let client = Client::new(&s3_config);

        info!("S3 file storage client initialized.");

        Ok(Self {
            client,
            bucket: s3_bucket,
        })
    }

    pub async fn write_file(&self, key: &str, bytes: Vec<u8>) -> anyhow::Result<()> {
        let body = ByteStream::from(bytes);

        let res = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .acl(ObjectCannedAcl::PublicRead)
            .content_type("image/jpeg")
            .send()
            .await;

        if let Err(e) = res {
            error!("Error uploading file {} to S3: {:?}", key, e);
            return Err(anyhow::anyhow!("S3 upload failed"));
        }

        info!("File {} written to S3", key);

        Ok(())
    }
}
