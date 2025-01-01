use crate::common::database::queries::*;
use crate::services::*;
use crate::types::*;
use rusqlite::types::Value;
use std::sync::Arc;

const TABLE: &str = "files";

pub struct FileRepository {
    db: Arc<dyn DatabaseInterface>,
}

impl FileRepository {
    pub async fn get(&self, id: u64) -> Result<FileRecord> {
        let query = SelectQuery {
            table_name: TABLE.to_string(),
            conditions: Attributes::from(&[("id".to_string(), Value::from(id as i64))]),
            ..Default::default()
        };

        match self.db.get_record(query).await {
            Ok(Some(props)) => FileRecord::from_attributes(&props),
            Ok(None) => Err(Error::FileNotFound),
            Err(err) => Err(err),
        }
    }
}

impl Locatable for FileRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
