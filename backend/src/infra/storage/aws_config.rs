use crate::types::Result;
use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_credential_types::Credentials;
use aws_sdk_s3::config::SharedCredentialsProvider;

pub struct AwsConfig {}

impl AwsConfig {
    pub fn files(key: &str, secret: &str, region: &str, endpoint: &str) -> Result<SdkConfig> {
        let credentials = Credentials::new(key, secret, None, None, Self::get_app());
        let credentials = SharedCredentialsProvider::new(credentials);

        let timeout = aws_config::timeout::TimeoutConfig::builder()
            .connect_timeout(std::time::Duration::from_secs(30))
            .build();

        let config = SdkConfig::builder()
            .region(Region::new(region.to_string()))
            .credentials_provider(credentials)
            .timeout_config(timeout)
            .behavior_version(BehaviorVersion::latest())
            .endpoint_url(endpoint)
            .build();

        Ok(config)
    }

    fn get_app() -> &'static str {
        env!("CARGO_PKG_NAME")
    }
}
