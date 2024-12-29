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
        let query = InsertQuery {
            table_name: TABLE.to_string(),
            attributes: Attributes::from([
                ("id".to_string(), Value::from(record.id as i64)),
                ("user_id".to_string(), Value::from(record.user_id as i64)),
                ("added_at".to_string(), Value::from(record.added_at as i64)),
                ("result".to_string(), Value::from(record.result)),
            ]),
        };

        self.db.add_record(query).await
    }
}

impl Locatable for TrainingRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
