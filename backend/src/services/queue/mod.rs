/**
 * Simple message queue implementation.
 *
 * Messages are stored in the database table, queue_messages.
 * This class only adds, retrieves and removes text messages,
 * parsing and processing happens outside.
 */
use log::debug;
use std::sync::Arc;

use crate::services::Database;
use crate::types::{QueueMessage, Result};
use crate::utils::{get_timestamp, get_unique_id};

/**
 * Message delay in seconds per failed attempts.
 * Each new failure increases the delay: 60 seconds first time, 120 second time, etc.
 */
const DELAY: u64 = 60;

pub struct QueueService {
    db: Arc<dyn Database>,
    delay: u64,
}

impl QueueService {
    pub fn new(db: &Arc<dyn Database>) -> Result<Self> {
        Ok(Self {
            db: db.clone(),
            delay: DELAY,
        })
    }

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

        self.db.add_queue_message(&msg).await?;

        debug!("Message {} added to queue, payload: {}", id, payload);

        Ok(msg)
    }

    pub async fn pop(&self) -> Result<Option<QueueMessage>> {
        let msg = self.db.pick_queue_message().await?;

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
        self.db.delete_queue_message(msg.id).await?;
        debug!("Message {} deleted from queue.", msg.id);
        Ok(())
    }

    async fn delay_message(&self, msg: &QueueMessage) -> Result<()> {
        let now = get_timestamp();
        self.db
            .delay_queue_message(msg.id, now + self.delay * (msg.attempts + 1))
            .await?;
        debug!("Message {} delayed for {} seconds.", msg.id, self.delay);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::database::SqliteDatabase;
    use env_logger;
    use std::env;

    async fn setup() -> QueueService {
        env::set_var("TREEMAP_SQLITE_PATH", ":memory:");

        if let Err(_) = env_logger::try_init() {
            debug!("env_logger already initialized.");
        };

        let db = SqliteDatabase::new()
            .await
            .expect("Error initializing database.");
        let dbh: Arc<dyn Database> = Arc::new(db);

        QueueService::new(&dbh).expect("Error creating queue service.")
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

        let msg = queue.pop().await.expect("Error receiving message.");
        assert!(msg.is_some());
        assert_eq!(msg.unwrap().payload, "test message");
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
