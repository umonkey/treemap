use crate::config::Secrets;
use crate::types::Result;
use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_credential_types::Credentials;
use aws_sdk_s3::config::SharedCredentialsProvider;
use std::sync::Arc;

pub struct AwsConfig {}

impl AwsConfig {
    pub fn files(secrets: Arc<Secrets>) -> Result<SdkConfig> {
        let key = secrets.require("FILES_KEY")?;
        let secret = secrets.require("FILES_SECRET")?;
        let region = secrets.require("FILES_REGION")?;

        let credentials = Credentials::new(&key, &secret, None, None, Self::get_app());
        let credentials = SharedCredentialsProvider::new(credentials);

        let timeout = aws_config::timeout::TimeoutConfig::builder()
            .connect_timeout(std::time::Duration::from_secs(30))
            .build();

        let config = SdkConfig::builder()
            .region(Region::new(region))
            .credentials_provider(credentials)
            .timeout_config(timeout)
            .behavior_version(BehaviorVersion::latest())
            .build();

        Ok(config)
    }

    fn get_app() -> &'static str {
        env!("CARGO_PKG_NAME")
    }
}
