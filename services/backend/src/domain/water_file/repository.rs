//! Access to the `water_source_files` link table.

use super::models::WaterFile;
use crate::infra::database::{Database, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use std::sync::Arc;

pub struct WaterFileRepository {
    db: Arc<Database>,
}

impl WaterFileRepository {
    pub async fn add(&self, link: &WaterFile) -> Result<bool> {
        let sql = "INSERT OR IGNORE INTO `water_source_files` (`id`, `water_source_id`, `file_id`, `added_at`, `added_by`) VALUES (?, ?, ?, ?, ?)";

        let params = [
            Value::from(link.id as i64),
            Value::from(link.water_source_id as i64),
            Value::from(link.file_id as i64),
            Value::from(link.added_at as i64),
            Value::from(link.added_by as i64),
        ];

        let rows = self.db.execute_sql(sql, &params).await?;

        Ok(rows > 0)
    }
}

impl Injectable for WaterFileRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}
