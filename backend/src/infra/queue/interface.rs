//! Queue client factory.
//!
//! This creates the right queue driver according to the configuration
//! and provides transparent access to it.

use super::base::BaseQueueInterface;
use super::database_queue::DatabaseQueue;
use super::sqs_queue::SqsQueue;
use super::types::QueueMessage;
use crate::config::Config;
use crate::services::{Locatable, Locator};
use crate::types::Result;
use std::sync::Arc;

pub struct Queue {
    queue: Arc<dyn BaseQueueInterface>,
}

impl Locatable for Queue {
    fn create(locator: &Locator) -> Result<Self> {
        let config = locator.get::<Config>()?;

        let queue: Arc<dyn BaseQueueInterface> = if config.sqs_queue.is_some() {
            locator.get::<SqsQueue>()?
        } else {
            locator.get::<DatabaseQueue>()?
        };

        Ok(Self { queue })
    }
}

impl Queue {
    pub async fn push(&self, payload: &str) -> Result<QueueMessage> {
        self.queue.push(payload).await
    }

    pub async fn pop(&self) -> Result<Option<QueueMessage>> {
        self.queue.pop().await
    }

    pub async fn delete(&self, msg: &QueueMessage) -> Result<()> {
        self.queue.delete(msg).await
    }

    pub async fn delay(&self, msg: &QueueMessage) -> Result<()> {
        self.queue.delay(msg).await
    }
}
