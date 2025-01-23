use crate::common::database::queries::*;
use crate::services::*;
use crate::types::*;
use crate::utils::get_timestamp;
use rusqlite::types::Value;
use std::sync::Arc;

const TABLE: &str = "queue_messages";

pub struct QueueMessageRepository {
    db: Arc<dyn DatabaseInterface>,
}

impl QueueMessageRepository {
    pub async fn pick(&self) -> Result<Option<QueueMessage>> {
        let now = get_timestamp();

        let query = SelectQuery::new(TABLE).with_order("id");

        let records = self.query_multiple(query).await?;

        for record in records {
            if record.attempts < 10 && record.available_at <= now {
                return Ok(Some(record));
            }
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

    async fn query_multiple(&self, query: SelectQuery) -> Result<Vec<QueueMessage>> {
        let records = self.db.get_records(query).await?;

        Ok(records
            .iter()
            .map(|props| QueueMessage::from_attributes(props).unwrap())
            .collect())
    }
}

impl Locatable for QueueMessageRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
