use crate::common::database::queries::InsertQuery;
use crate::services::*;
use crate::types::*;
use rusqlite::types::Value;
use std::sync::Arc;

const TABLE: &str = "training";

pub struct TrainingRepository {
    db: Arc<dyn DatabaseInterface>,
}

impl TrainingRepository {
    pub async fn add(&self, record: &TrainingRecord) -> Result<()> {
        let query = InsertQuery::new(TABLE)
            .with_value("id", Value::from(record.id as i64))
            .with_value("user_id", Value::from(record.user_id as i64))
            .with_value("added_at", Value::from(record.added_at as i64))
            .with_value("result", Value::from(record.result));

        self.db.add_record(query).await
    }
}

impl Locatable for TrainingRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
