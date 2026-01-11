//! Access to the `uploads` table, where temporary file uploads are stored.
//! The uploads are then used to create tree photos, by providing ids during
//! tree creation or manipulation.

use super::models::Upload;
use crate::infra::database::{Database, DeleteQuery, InsertQuery, SelectQuery, Value};
use crate::services::{Locatable, Locator};
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "uploads";

pub struct UploadRepository {
    db: Arc<Database>,
}

impl UploadRepository {
    #[allow(unused)]
    pub async fn get(&self, id: u64) -> Result<Option<Upload>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));

        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(Upload::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub async fn add(&self, file: &Upload) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(file.to_attributes());
        self.db.add_record(query).await?;
        Ok(())
    }

    pub async fn delete(&self, file: &Upload) -> Result<()> {
        let query = DeleteQuery::new(TABLE).with_condition("id", Value::from(file.id as i64));

        self.db.delete(query).await
    }
}

impl Locatable for UploadRepository {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<Database>()?;
        Ok(Self { db })
    }
}
