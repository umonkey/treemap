//! Access to the `files` table, where tree photos are stored.

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
    pub async fn get(&self, id: u64) -> Result<Option<FileRecord>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));

        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(FileRecord::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub async fn add(&self, file: &FileRecord) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(file.to_attributes());

        self.db.add_record(query).await?;

        Ok(())
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

    pub async fn find_by_tree(&self, tree_id: u64) -> Result<Vec<FileRecord>> {
        let query = SelectQuery::new(TABLE).with_condition("tree_id", Value::from(tree_id as i64));
        self.query_multiple(query).await
    }

    async fn query_multiple(&self, query: SelectQuery) -> Result<Vec<FileRecord>> {
        let records = self.db.get_records(query).await?;

        records
            .iter()
            .map(|props| FileRecord::from_attributes(props).map_err(|_| Error::DatabaseStructure))
            .collect()
    }
}

impl Locatable for FileRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
