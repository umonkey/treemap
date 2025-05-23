//! Access to the `uploads` table, where temporary file uploads are stored.
//! The uploads are then used to create tree photos, by providing ids during
//! tree creation or manipulation.

use crate::common::database::queries::*;
use crate::services::*;
use crate::types::*;
use rusqlite::types::Value;
use std::sync::Arc;

const TABLE: &str = "uploads";

pub struct UploadRepository {
    db: Arc<dyn DatabaseInterface>,
}

impl UploadRepository {
    #[allow(unused)]
    pub async fn get(&self, id: u64) -> Result<Option<UploadRecord>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));

        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(UploadRecord::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub async fn add(&self, file: &UploadRecord) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(file.to_attributes());
        self.db.add_record(query).await?;
        Ok(())
    }
}

impl Locatable for UploadRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
