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

    pub async fn update(&self, file: &FileRecord) -> Result<()> {
        let query = UpdateQuery {
            table_name: TABLE.to_string(),
            conditions: Attributes::from(&[("id".to_string(), Value::from(file.id as i64))]),
            attributes: Attributes::from(&[
                ("added_at".to_string(), Value::from(file.added_at as i64)),
                ("added_by".to_string(), Value::from(file.added_by as i64)),
                (
                    "deleted_at".to_string(),
                    Value::from(file.deleted_at.unwrap_or(0) as i64),
                ),
                (
                    "deleted_by".to_string(),
                    Value::from(file.deleted_by.unwrap_or(0) as i64),
                ),
                ("small_id".to_string(), Value::from(file.small_id as i64)),
                ("large_id".to_string(), Value::from(file.large_id as i64)),
            ]),
        };

        self.db.update(query).await?;

        Ok(())
    }

    #[allow(unused)]
    pub async fn delete(&self, file: &FileRecord) -> Result<()> {
        let query = DeleteQuery {
            table_name: TABLE.to_string(),
            conditions: Attributes::from(&[("id".to_string(), Value::from(file.id as i64))]),
        };

        self.db.delete(query).await
    }
}

impl Locatable for FileRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
