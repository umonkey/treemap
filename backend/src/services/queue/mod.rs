//! Simple message queue implementation.
//!
//! Messages are stored in the database table, queue_messages.
//! This class only adds, retrieves and removes text messages,
//! parsing and processing happens outside.

use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use log::debug;
use std::sync::Arc;

/**
 * Message delay in seconds per failed attempts.
 * Each new failure increases the delay: 60 seconds first time, 120 second time, etc.
 */
const DELAY: u64 = 60;

pub struct QueueService {
    messages: Arc<QueueMessageRepository>,
    delay: u64,
}

impl QueueService {
    pub async fn push(&self, payload: &str) -> Result<QueueMessage> {
        let id = get_unique_id()?;
        let now = get_timestamp();

        let msg = QueueMessage {
            id,
            added_at: now,
            available_at: now,
            payload: payload.to_string(),
            attempts: 0,
        };

        self.messages.add(&msg).await?;

        debug!("Message {id} added to queue, payload: {payload}");

        Ok(msg)
    }

    pub async fn pop(&self) -> Result<Option<QueueMessage>> {
        let msg = self.messages.pick().await?;

        match msg {
            Some(msg) => {
                debug!(
                    "Message {} popped from queue, payload: {}",
                    msg.id, msg.payload
                );
                self.delay_message(&msg).await?;
                Ok(Some(msg))
            }

            None => Ok(None),
        }
    }

    pub async fn delete(&self, msg: &QueueMessage) -> Result<()> {
        self.messages.delete(msg).await?;
        debug!("Message {} deleted from queue.", msg.id);
        Ok(())
    }

    async fn delay_message(&self, msg: &QueueMessage) -> Result<()> {
        let now = get_timestamp();
        self.messages
            .delay(msg, now + self.delay * (msg.attempts + 1))
            .await?;
        debug!("Message {} delayed for {} seconds.", msg.id, self.delay);
        Ok(())
    }
}

impl Locatable for QueueService {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            messages: locator.get::<QueueMessageRepository>()?,
            delay: DELAY,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup() -> Arc<QueueService> {
        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        Locator::new()
            .get::<QueueService>()
            .expect("Error creating queue service.")
    }

    #[tokio::test]
    async fn test_pop_empty() {
        let queue = setup().await;

        let msg = queue.pop().await.expect("Error receiving message.");
        assert!(msg.is_none());
    }

    #[tokio::test]
    async fn test_push_pop() {
        let queue = setup().await;

        queue
            .push("test message")
            .await
            .expect("Error adding message.");

        let msg = queue
            .pop()
            .await
            .expect("Error receiving message.")
            .expect("Empty message received.");

        assert_eq!(msg.payload, "test message");
    }

    #[tokio::test]
    async fn test_delay() {
        let queue = setup().await;

        queue
            .push("test message")
            .await
            .expect("Error adding message.");

        let take1 = queue.pop().await.expect("Error receiving message.");
        assert!(take1.is_some());

        let take2 = queue.pop().await.expect("Error receiving message.");
        assert!(take2.is_none());
    }

    #[tokio::test]
    async fn test_delete() {
        let queue = setup().await;

        let msg = queue
            .push("test message")
            .await
            .expect("Error adding message.");

        queue.delete(&msg).await.expect("Error deleting message.");
    }
}
