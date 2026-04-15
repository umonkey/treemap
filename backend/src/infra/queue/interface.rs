//! Queue client factory.
//!
//! This creates the right queue driver according to the configuration
//! and provides transparent access to it.

use super::base::BaseQueueInterface;
use super::database_queue::DatabaseQueue;
use super::sqs_queue::SqsQueue;
use super::types::QueueMessage;
use crate::infra::config::Config;
use crate::infra::database::Database;
use crate::infra::secrets::Secrets;
use crate::types::Result;
use std::sync::Arc;

pub struct Queue {
    queue: Arc<dyn BaseQueueInterface>,
}

impl Queue {
    pub fn new(config: &Config, secrets: &Secrets, database: &Arc<Database>) -> Result<Self> {
        let queue: Arc<dyn BaseQueueInterface> = if config.sqs_url.is_some() {
            Arc::new(SqsQueue::new(config, secrets)?)
        } else {
            Arc::new(DatabaseQueue::new(database.clone()))
        };

        Ok(Self { queue })
    }

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
