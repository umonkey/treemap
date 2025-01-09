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
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));

        match self.db.get_record(query).await {
            Ok(Some(props)) => FileRecord::from_attributes(&props),
            Ok(None) => Err(Error::FileNotFound),
            Err(err) => Err(err),
        }
    }

    pub async fn update(&self, file: &FileRecord) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(file.id as i64))
            .with_value("added_at", Value::from(file.added_at as i64))
            .with_value("added_by", Value::from(file.added_by as i64))
            .with_value(
                "deleted_at",
                Value::from(file.deleted_at.unwrap_or(0) as i64),
            )
            .with_value(
                "deleted_by",
                Value::from(file.deleted_by.unwrap_or(0) as i64),
            )
            .with_value("small_id", Value::from(file.small_id as i64))
            .with_value("large_id", Value::from(file.large_id as i64));

        self.db.update(query).await?;

        Ok(())
    }

    #[allow(unused)]
    pub async fn delete(&self, file: &FileRecord) -> Result<()> {
        let query = DeleteQuery::new(TABLE).with_condition("id", Value::from(file.id as i64));

        self.db.delete(query).await
    }
}

impl Locatable for FileRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
