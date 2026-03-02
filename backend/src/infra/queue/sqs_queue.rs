//! SQS-based queue implementation.

use super::aws_config::AwsConfig;
use super::base::BaseQueueInterface;
use super::types::QueueMessage;
use crate::infra::config::Config;
use crate::infra::secrets::Secrets;
use crate::services::{Locatable, Locator};
use crate::types::{Error, Result};
use crate::utils::get_timestamp;
use async_trait::async_trait;
use aws_sdk_sqs::types::MessageSystemAttributeName;
use log::{debug, error};

pub struct SqsQueue {
    client: aws_sdk_sqs::Client,
    queue_url: String,
}

#[async_trait]
impl BaseQueueInterface for SqsQueue {
    async fn push(&self, payload: &str) -> Result<QueueMessage> {
        let output = self
            .client
            .send_message()
            .queue_url(&self.queue_url)
            .message_body(payload)
            .send()
            .await
            .map_err(|e| {
                error!("Error sending message to SQS: {e:?}");
                Error::Queue
            })?;

        let id = output.message_id.unwrap_or_default();
        debug!("Message {id} added to SQS queue.");

        Ok(QueueMessage {
            id,
            added_at: get_timestamp(),
            available_at: get_timestamp(),
            payload: payload.to_string(),
            attempts: 0,
        })
    }

    async fn pop(&self) -> Result<Option<QueueMessage>> {
        let output = self
            .client
            .receive_message()
            .queue_url(&self.queue_url)
            .max_number_of_messages(1)
            .message_system_attribute_names(MessageSystemAttributeName::SentTimestamp)
            .message_system_attribute_names(MessageSystemAttributeName::ApproximateReceiveCount)
            .wait_time_seconds(10)
            .send()
            .await
            .map_err(|e| {
                error!("Error receiving message from SQS: {e:?}");
                Error::Queue
            })?;

        if let Some(messages) = output.messages {
            if let Some(msg) = messages.into_iter().next() {
                let receipt_handle = msg.receipt_handle.ok_or(Error::Queue)?;
                let payload = msg.body.unwrap_or_default();

                let added_at = msg
                    .attributes
                    .as_ref()
                    .and_then(|attrs| attrs.get(&MessageSystemAttributeName::SentTimestamp))
                    .and_then(|v| v.parse::<u64>().ok())
                    .map(|ts| ts / 1000)
                    .unwrap_or_else(get_timestamp);

                let attempts = msg
                    .attributes
                    .as_ref()
                    .and_then(|attrs| {
                        attrs.get(&MessageSystemAttributeName::ApproximateReceiveCount)
                    })
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(1);

                return Ok(Some(QueueMessage {
                    id: receipt_handle,
                    added_at,
                    available_at: get_timestamp(),
                    payload,
                    attempts,
                }));
            }
        }

        Ok(None)
    }

    async fn delete(&self, msg: &QueueMessage) -> Result<()> {
        self.client
            .delete_message()
            .queue_url(&self.queue_url)
            .receipt_handle(&msg.id)
            .send()
            .await
            .map_err(|e| {
                error!("Error deleting message from SQS: {e:?}");
                Error::Queue
            })?;

        debug!("Message {} deleted from SQS.", msg.id);
        Ok(())
    }

    async fn delay(&self, msg: &QueueMessage) -> Result<()> {
        // Linear backoff: 60s * attempts
        let visibility_timeout = 60 * (msg.attempts + 1) as i32;

        self.client
            .change_message_visibility()
            .queue_url(&self.queue_url)
            .receipt_handle(&msg.id)
            .visibility_timeout(visibility_timeout)
            .send()
            .await
            .map_err(|e| {
                error!("Error changing visibility for SQS message: {e:?}");
                Error::Queue
            })?;

        debug!(
            "Message {} delayed for {} seconds.",
            msg.id, visibility_timeout
        );
        Ok(())
    }
}

impl Locatable for SqsQueue {
    fn create(locator: &Locator) -> Result<Self> {
        let config = locator.get::<Config>()?;
        let secrets = locator.get::<Secrets>()?;

        let sqs_url = config
            .sqs_url
            .clone()
            .ok_or(Error::Config("sqs_url not set".to_string()))?;

        let url = url::Url::parse(&sqs_url)
            .map_err(|e| Error::Config(format!("Invalid sqs_url: {e}")))?;

        let host = url.host_str().unwrap_or_default();

        let region = if host.starts_with("sqs.") && host.ends_with(".amazonaws.com") {
            host.split('.').nth(1).unwrap_or("us-east-1").to_string()
        } else {
            "us-east-1".to_string()
        };

        let endpoint = format!(
            "{}://{}{}",
            url.scheme(),
            host,
            if let Some(port) = url.port() {
                format!(":{port}")
            } else {
                "".to_string()
            }
        );

        let sqs_key = secrets
            .sqs_key
            .clone()
            .ok_or(Error::Config("Secret SQS_KEY not set".to_string()))?;

        let sqs_secret = secrets
            .sqs_secret
            .clone()
            .ok_or(Error::Config("Secret SQS_SECRET not set".to_string()))?;

        let aws_config = AwsConfig::sqs(&sqs_key, &sqs_secret, &region, &endpoint)?;

        let client = aws_sdk_sqs::Client::new(&aws_config);

        Ok(Self {
            client,
            queue_url: sqs_url,
        })
    }
}
