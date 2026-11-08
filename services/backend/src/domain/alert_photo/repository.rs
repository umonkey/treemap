use super::models::AlertPhoto;
use crate::infra::database::{Database, SelectQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "chatbot_alerts_photos";

pub struct AlertPhotoRepository {
    db: Arc<Database>,
}

impl AlertPhotoRepository {
    pub async fn get_by_alert(&self, alert_id: u64) -> Result<Vec<AlertPhoto>> {
        let query =
            SelectQuery::new(TABLE).with_condition("alert_id", Value::from(alert_id as i64));

        self.query_multiple(query).await
    }

    pub async fn get_by_alert_ids(
        &self,
        alert_ids: &[u64],
    ) -> Result<std::collections::HashMap<u64, Vec<String>>> {
        if alert_ids.is_empty() {
            return Ok(std::collections::HashMap::new());
        }

        let placeholders: Vec<String> = alert_ids.iter().map(|_| "?".to_string()).collect();

        let placeholders_str = placeholders.join(", ");

        let sql = format!(
            "SELECT * FROM `{}` WHERE alert_id IN ({})",
            TABLE, placeholders_str
        );

        let mut params = Vec::new();

        for id in alert_ids {
            params.push(Value::from(*id as i64));
        }

        let records = self.db.fetch_sql(&sql, &params).await?;

        let mut map: std::collections::HashMap<u64, Vec<String>> = std::collections::HashMap::new();

        for record in records {
            let photo = AlertPhoto::from_attributes(&record)?;

            map.entry(photo.alert_id)
                .or_default()
                .push(photo.photo_path);
        }

        Ok(map)
    }

    async fn query_multiple(&self, query: SelectQuery) -> Result<Vec<AlertPhoto>> {
        let records = self.db.get_records(query).await?;
        records.iter().map(AlertPhoto::from_attributes).collect()
    }
}

impl Injectable for AlertPhotoRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}
