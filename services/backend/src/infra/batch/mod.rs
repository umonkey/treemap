use crate::infra::config::Config;
use crate::infra::secrets::Secrets;
use crate::services::{Context, Injectable};
use crate::types::{Error, Result};
use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_credential_types::Credentials;
use aws_sdk_batch::config::SharedCredentialsProvider;
use aws_sdk_batch::types::{ContainerOverrides, KeyValuePair};
use log::error;

pub struct BatchClient {
    client: aws_sdk_batch::Client,
    job_queue: String,
    files_key: String,
    files_secret: String,
    files_region: String,
    files_endpoint: String,
}

impl BatchClient {
    pub fn new(config: &Config, secrets: &Secrets) -> Result<Self> {
        let job_queue = config
            .batch_job_queue
            .clone()
            .unwrap_or_else(|| "treemap-transcode".to_string());

        let aws_key = secrets
            .aws_key
            .clone()
            .ok_or_else(|| Error::Config("AWS_KEY secret not set".to_string()))?;

        let aws_secret = secrets
            .aws_secret
            .clone()
            .ok_or_else(|| Error::Config("AWS_SECRET secret not set".to_string()))?;

        let aws_region = secrets
            .aws_region
            .clone()
            .ok_or_else(|| Error::Config("AWS_REGION secret not set".to_string()))?;

        let files_key = secrets.files_key.clone().unwrap_or_default();
        let files_secret = secrets.files_secret.clone().unwrap_or_default();
        let files_region = config
            .files_region
            .clone()
            .unwrap_or_else(|| "us-east-1".to_string());
        let files_endpoint = config.files_endpoint.clone().unwrap_or_default();

        let credentials =
            Credentials::new(&aws_key, &aws_secret, None, None, env!("CARGO_PKG_NAME"));
        let credentials = SharedCredentialsProvider::new(credentials);

        let timeout = aws_config::timeout::TimeoutConfig::builder()
            .connect_timeout(std::time::Duration::from_secs(30))
            .build();

        let sdk_config = SdkConfig::builder()
            .region(Region::new(aws_region.clone()))
            .credentials_provider(credentials)
            .timeout_config(timeout)
            .behavior_version(BehaviorVersion::latest())
            .build();

        let client = aws_sdk_batch::Client::new(&sdk_config);

        Ok(Self {
            client,
            job_queue,
            files_key,
            files_secret,
            files_region,
            files_endpoint,
        })
    }

    pub async fn transcode(
        &self,
        job_name: &str,
        input_path: &str,
        output_path: &str,
    ) -> Result<String> {
        let envs = vec![
            KeyValuePair::builder()
                .name("AWS_ACCESS_KEY_ID")
                .value(&self.files_key)
                .build(),
            KeyValuePair::builder()
                .name("AWS_SECRET_ACCESS_KEY")
                .value(&self.files_secret)
                .build(),
            KeyValuePair::builder()
                .name("AWS_REGION")
                .value(&self.files_region)
                .build(),
            KeyValuePair::builder()
                .name("AWS_ENDPOINT_URL")
                .value(&self.files_endpoint)
                .build(),
            KeyValuePair::builder()
                .name("INPUT_PATH")
                .value(input_path)
                .build(),
            KeyValuePair::builder()
                .name("OUTPUT_PATH")
                .value(output_path)
                .build(),
        ];

        let container_overrides = ContainerOverrides::builder()
            .command(input_path)
            .command(output_path)
            .set_environment(Some(envs))
            .build();

        let output = self
            .client
            .submit_job()
            .job_name(job_name)
            .job_queue(&self.job_queue)
            .job_definition("treemap-transcoder")
            .container_overrides(container_overrides)
            .send()
            .await
            .map_err(|e| {
                error!("Error submitting batch job {job_name}: {e}");
                Error::Config(format!("Failed to submit batch job: {e}"))
            })?;

        let arn = output.job_arn().ok_or_else(|| {
            Error::Config("Batch job submission did not return job ARN".to_string())
        })?;

        Ok(arn.to_string())
    }

    #[allow(dead_code)]
    pub async fn extract(
        &self,
        job_name: &str,
        gpx_offset: f64,
        mask_size: f64,
        dataset_url: &str,
        result_url: &str,
    ) -> Result<String> {
        let envs = vec![
            KeyValuePair::builder()
                .name("AWS_ACCESS_KEY_ID")
                .value(&self.files_key)
                .build(),
            KeyValuePair::builder()
                .name("AWS_SECRET_ACCESS_KEY")
                .value(&self.files_secret)
                .build(),
            KeyValuePair::builder()
                .name("AWS_REGION")
                .value(&self.files_region)
                .build(),
            KeyValuePair::builder()
                .name("AWS_ENDPOINT_URL")
                .value(&self.files_endpoint)
                .build(),
            KeyValuePair::builder()
                .name("GPX_OFFSET")
                .value(gpx_offset.to_string())
                .build(),
            KeyValuePair::builder()
                .name("MASK_SIZE")
                .value(mask_size.to_string())
                .build(),
            KeyValuePair::builder()
                .name("DATASET_URL")
                .value(dataset_url)
                .build(),
            KeyValuePair::builder()
                .name("RESULT_URL")
                .value(result_url)
                .build(),
        ];

        let container_overrides = ContainerOverrides::builder()
            .command("bin/process")
            .set_environment(Some(envs))
            .build();

        let output = self
            .client
            .submit_job()
            .job_name(job_name)
            .job_queue(&self.job_queue)
            .job_definition("treemap-extractor")
            .container_overrides(container_overrides)
            .send()
            .await
            .map_err(|e| {
                error!("Error submitting batch job {job_name}: {e}");
                Error::Config(format!("Failed to submit batch job: {e}"))
            })?;

        let arn = output.job_arn().ok_or_else(|| {
            Error::Config("Batch job submission did not return job ARN".to_string())
        })?;

        Ok(arn.to_string())
    }

    pub async fn get_job_status(&self, arn: &str) -> Result<String> {
        let output = self
            .client
            .describe_jobs()
            .jobs(arn)
            .send()
            .await
            .map_err(|e| {
                error!("Error describing batch job {arn}: {e}");
                Error::Config(format!("Failed to describe batch job: {e}"))
            })?;

        let job = output
            .jobs()
            .first()
            .ok_or_else(|| Error::Config(format!("Batch job not found: {arn}")))?;

        let status = job
            .status()
            .map(|s| s.as_str().to_string())
            .unwrap_or_else(|| "UNKNOWN".to_string());
        Ok(status)
    }
}

impl Injectable for BatchClient {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        let config = ctx.config();
        let secrets = ctx.secrets();
        Self::new(&config, &secrets)
    }
}
