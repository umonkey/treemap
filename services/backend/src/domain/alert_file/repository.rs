//! Access to the `chatbot_alerts_files` link table.

use super::models::AlertFile;
use crate::infra::database::{Database, SelectQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "chatbot_alerts_files";

pub struct AlertFileRepository {
    db: Arc<Database>,
}

impl AlertFileRepository {
    pub async fn add(&self, link: &AlertFile) -> Result<bool> {
        let sql = "INSERT OR IGNORE INTO `chatbot_alerts_files` (`id`, `alert_id`, `file_id`, `added_at`, `added_by`) VALUES (?, ?, ?, ?, ?)";

        let params = [
            Value::from(link.id as i64),
            Value::from(link.alert_id as i64),
            Value::from(link.file_id as i64),
            Value::from(link.added_at as i64),
            Value::from(link.added_by as i64),
        ];

        let rows = self.db.execute_sql(sql, &params).await?;

        Ok(rows > 0)
    }

    pub async fn find_by_alert(&self, alert_id: u64) -> Result<Vec<AlertFile>> {
        let query =
            SelectQuery::new(TABLE).with_condition("alert_id", Value::from(alert_id as i64));

        let records = self.db.get_records(query).await?;

        records.iter().map(AlertFile::from_attributes).collect()
    }
}

impl Injectable for AlertFileRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}
