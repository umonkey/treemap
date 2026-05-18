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
