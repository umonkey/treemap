use super::types::QueueMessage;
use crate::types::Result;
use async_trait::async_trait;

#[async_trait]
pub trait BaseQueueInterface: Send + Sync {
    async fn push(&self, payload: &str) -> Result<QueueMessage>;
    async fn pop(&self) -> Result<Option<QueueMessage>>;
    async fn delete(&self, msg: &QueueMessage) -> Result<()>;
    async fn delay(&self, msg: &QueueMessage) -> Result<()>;
}
