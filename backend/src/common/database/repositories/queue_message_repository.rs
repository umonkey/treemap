use crate::common::database::queries::*;
use crate::infra::database::Value;
use crate::services::*;
use crate::types::*;
use crate::utils::get_timestamp;
use std::sync::Arc;

const TABLE: &str = "queue_messages";

pub struct QueueMessageRepository {
    db: Arc<dyn DatabaseInterface>,
}

impl QueueMessageRepository {
    pub async fn pick(&self) -> Result<Option<QueueMessage>> {
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

    pub async fn add(&self, msg: &QueueMessage) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(msg.to_attributes());
        self.db.add_record(query).await?;
        Ok(())
    }

    pub async fn delay(&self, msg: &QueueMessage, available_at: u64) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(msg.id as i64))
            .with_value("available_at", Value::from(available_at as i64))
            .with_value("attempts", Value::from(msg.attempts as i64 + 1));

        self.db.update(query).await?;

        Ok(())
    }

    pub async fn delete(&self, msg: &QueueMessage) -> Result<()> {
        let query = DeleteQuery::new(TABLE).with_condition("id", Value::from(msg.id as i64));
        self.db.delete(query).await
    }
}

impl Locatable for QueueMessageRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
