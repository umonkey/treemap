/**
 * Simple message queue implementation.
 * Messages are stored in the database table, queue_messages.
 */

use std::sync::Arc;

use crate::services::Database;
use crate::utils::{get_timestamp, get_unique_id};
use crate::types::QueueMessage;

pub struct QueueService {
    db: Arc<dyn Database>,
}

impl QueueService {
    pub fn init(db: &Arc<dyn Database>) -> Result<Self> {
        Ok(Self {
            db: db.clone(),
        })
    }

    pub async fn push(&self, payload: &str) -> Result<u64> {
        let id = get_unique_id()?;
        let now = get_timestamp();

        let msg = QueueMessage {
            id,
            added_at: now,
            available_at: now,
            payload: payload.to_string(),
        };

        self.db.add_queue_message(&msg).await?;

        Ok(id)
    }
}
