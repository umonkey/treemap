//! Reads AWS Batch job logs from CloudWatch Logs.

use crate::infra::config::Config;
use crate::infra::secrets::Secrets;
use crate::types::{Error, Result};
use aws_config::{BehaviorVersion, Region, SdkConfig};
use aws_credential_types::Credentials;
use aws_sdk_cloudwatchlogs::config::SharedCredentialsProvider;
use chrono::{DateTime, Utc};
use log::error;

/// AWS Batch writes job logs to this group unless the compute environment
/// overrides it.
const DEFAULT_LOG_GROUP: &str = "/aws/batch/job";

/// Maximum number of events requested per page.
const PAGE_LIMIT: i32 = 10_000;

/// Maximum size of the rendered log, in bytes.
const MAX_LOG_BYTES: usize = 50 * 1024 * 1024;

pub struct LogsClient {
    client: aws_sdk_cloudwatchlogs::Client,
    log_group: String,
}

impl LogsClient {
    pub fn new(config: &Config, secrets: &Secrets) -> Result<Self> {
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

        let log_group = config
            .batch_log_group
            .clone()
            .unwrap_or_else(|| DEFAULT_LOG_GROUP.to_string());

        let credentials =
            Credentials::new(&aws_key, &aws_secret, None, None, env!("CARGO_PKG_NAME"));
        let credentials = SharedCredentialsProvider::new(credentials);

        let timeout = aws_config::timeout::TimeoutConfig::builder()
            .connect_timeout(std::time::Duration::from_secs(30))
            .build();

        let sdk_config = SdkConfig::builder()
            .region(Region::new(aws_region))
            .credentials_provider(credentials)
            .timeout_config(timeout)
            .behavior_version(BehaviorVersion::latest())
            .build();

        let client = aws_sdk_cloudwatchlogs::Client::new(&sdk_config);

        Ok(Self { client, log_group })
    }

    /// Fetches the whole log stream and renders it as text.
    ///
    /// An empty stream is not an error: it produces a header-only file so a
    /// completed job does not become a poison queue message.
    pub async fn fetch(&self, stream: &str) -> Result<String> {
        let mut output = format!("# Log stream: {stream}\n");
        let mut token: Option<String> = None;
        let mut event_count: usize = 0;
        let mut truncated = false;

        loop {
            let mut request = self
                .client
                .filter_log_events()
                .log_group_name(&self.log_group)
                .log_stream_names(stream)
                .limit(PAGE_LIMIT);

            if let Some(token) = &token {
                request = request.next_token(token);
            }

            let response = request.send().await.map_err(|e| {
                error!("Error fetching log stream {stream}: {e}");
                Error::FileDownload
            })?;

            for event in response.events() {
                event_count += 1;

                if output.len() >= MAX_LOG_BYTES {
                    truncated = true;
                    break;
                }

                output.push_str(&format_event(
                    event.timestamp(),
                    event.message().unwrap_or_default(),
                ));
            }

            if truncated {
                break;
            }

            let next = match response.next_token() {
                Some(next) if !next.is_empty() => next.to_string(),
                _ => break,
            };

            // Guard against a page that repeats the token it was requested with.
            if token.as_deref() == Some(next.as_str()) {
                break;
            }

            token = Some(next);
        }

        if event_count == 0 {
            output.push_str("[no log events]\n");
        } else if truncated {
            output.push_str("\n[log truncated]\n");
        }

        Ok(output)
    }
}

/// Renders one CloudWatch event as `[YYYY-MM-DD HH:MM:SS] message\n`.
fn format_event(timestamp: Option<i64>, message: &str) -> String {
    let timestamp = timestamp
        .and_then(DateTime::<Utc>::from_timestamp_millis)
        .map(|value| value.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "unknown time".to_string());

    format!("[{timestamp}] {message}\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_event_renders_timestamp_and_message() {
        // 2026-11-11T16:52:31Z
        assert_eq!(
            format_event(Some(1_794_415_951_000), "Starting job"),
            "[2026-11-11 16:52:31] Starting job\n"
        );
    }

    #[test]
    fn format_event_falls_back_without_timestamp() {
        assert_eq!(
            format_event(None, "plain text"),
            "[unknown time] plain text\n"
        );
    }
}
