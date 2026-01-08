use crate::config::{Config, Secrets};
use crate::types::{Error, Result};
use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_credential_types::Credentials;
use aws_sdk_s3::config::SharedCredentialsProvider;
use log::error;
use std::sync::Arc;

pub struct AwsConfig {}

impl AwsConfig {
    pub fn files(config: Arc<Config>, secrets: Arc<Secrets>) -> Result<SdkConfig> {
        let key = secrets
            .files_key
            .clone()
            .ok_or(Error::Config)
            .inspect_err(|_e| {
                error!("Secret FILES_KEY not set.");
            })?;

        let secret = secrets
            .files_secret
            .clone()
            .ok_or(Error::Config)
            .inspect_err(|_e| {
                error!("Secret FILES_SECRET not set.");
            })?;

        let region = config
            .files_region
            .clone()
            .ok_or(Error::Config)
            .inspect_err(|_e| {
                error!("Config option FILES_REGION not set.");
            })?;

        let endpoint = config
            .files_endpoint
            .clone()
            .ok_or(Error::Config)
            .inspect_err(|_e| {
                error!("Config option FILES_ENDPOIT not set.");
            })?;

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
            .endpoint_url(endpoint)
            .build();

        Ok(config)
    }

    fn get_app() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    pub fn sqs(config: Arc<Config>, secrets: Arc<Secrets>) -> Result<SdkConfig> {
        let key = secrets
            .sqs_key
            .clone()
            .ok_or(Error::Config)
            .inspect_err(|_e| {
                error!("Secret SQS_KEY not set.");
            })?;

        let secret = secrets
            .sqs_secret
            .clone()
            .ok_or(Error::Config)
            .inspect_err(|_e| {
                error!("Secret SQS_SECRET not set.");
            })?;

        let region = config
            .sqs_region
            .clone()
            .ok_or(Error::Config)
            .inspect_err(|_e| {
                error!("Config option SQS_REGION not set.");
            })?;

        let credentials = Credentials::new(&key, &secret, None, None, Self::get_app());
        let credentials = SharedCredentialsProvider::new(credentials);

        let timeout = aws_config::timeout::TimeoutConfig::builder()
            .connect_timeout(std::time::Duration::from_secs(30))
            .build();

        let mut builder = SdkConfig::builder()
            .region(Region::new(region))
            .credentials_provider(credentials)
            .timeout_config(timeout)
            .behavior_version(BehaviorVersion::latest());

        if let Some(endpoint) = config.sqs_endpoint.clone() {
            builder = builder.endpoint_url(endpoint);
        }

        Ok(builder.build())
    }
}
