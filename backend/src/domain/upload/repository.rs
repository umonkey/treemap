//! Access to the `uploads` table, where temporary file uploads are stored.
//! The uploads are then used to create tree photos, by providing ids during
//! tree creation or manipulation.

use super::models::Upload;
use crate::infra::database::{Database, InsertQuery, SelectQuery, UpdateQuery, Value};
use crate::services::{Context, Injectable};
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

    pub async fn finish(&self, id: u64, timestamp: u64) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(id as i64))
            .with_value("uploaded_at", Value::from(timestamp as i64));
        self.db.update(query).await?;
        Ok(())
    }
}

impl Injectable for UploadRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}
