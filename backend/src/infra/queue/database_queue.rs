//! This is the database based messaging queue.
//! It uses the whatever database was configured.

use super::base::BaseQueueInterface;
use super::types::QueueMessage;
use crate::infra::database::{Database, Value};
use crate::infra::database::{DeleteQuery, InsertQuery, UpdateQuery};
use crate::services::{Locatable, Locator};
use crate::types::Result;
use crate::utils::{get_timestamp, get_unique_id};
use async_trait::async_trait;
use log::debug;
use std::sync::Arc;

const TABLE: &str = "queue_messages";
const DELAY: u64 = 60;

pub struct DatabaseQueue {
    db: Arc<Database>,
}

// The actual queue logic for the shared interface.
#[async_trait]
impl BaseQueueInterface for DatabaseQueue {
    async fn push(&self, payload: &str) -> Result<QueueMessage> {
        let id = get_unique_id()?;
        let now = get_timestamp();

        let msg = QueueMessage {
            id: id.to_string(),
            added_at: now,
            available_at: now,
            payload: payload.to_string(),
            attempts: 0,
        };

        let query = InsertQuery::new(TABLE).with_values(msg.to_attributes());
        self.db.add_record(query).await?;

        debug!("Message {id} added to queue, payload: {payload}");

        Ok(msg)
    }

    async fn pop(&self) -> Result<Option<QueueMessage>> {
        let now = get_timestamp();

        let rows = self.db.sql(
            "SELECT * FROM queue_messages WHERE attempts < 10 AND available_at <= ? ORDER BY added_at LIMIT 1",
            &[Value::from(now)],
        ).await?;

        if let Some(row) = rows.into_iter().next() {
            return Ok(Some(QueueMessage::from_attributes(&row)?));
        }

        Ok(None)
    }

    async fn delete(&self, msg: &QueueMessage) -> Result<()> {
        let id = msg.id.parse::<i64>().unwrap_or_default();
        let query = DeleteQuery::new(TABLE).with_condition("id", Value::from(id));
        self.db.delete(query).await?;

        debug!("Message {} deleted from queue.", msg.id);

        Ok(())
    }

    async fn delay(&self, msg: &QueueMessage) -> Result<()> {
        let now = get_timestamp();
        let available_at = now + DELAY * msg.attempts + 1;
        let id = msg.id.parse::<i64>().unwrap_or_default();

        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(id))
            .with_value("available_at", Value::from(available_at))
            .with_value("attempts", Value::from(msg.attempts + 1));

        self.db.update(query).await?;

        debug!("Message {} delayed for {} seconds.", msg.id, DELAY);
        Ok(())
    }
}

impl Locatable for DatabaseQueue {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<Database>()?;
        let queue = Self { db };
        Ok(queue)
    }
}
