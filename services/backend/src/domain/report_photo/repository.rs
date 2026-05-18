use super::models::ReportPhoto;
use crate::infra::database::{Database, SelectQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "chatbot_report_photos";

pub struct ReportPhotoRepository {
    db: Arc<Database>,
}

impl ReportPhotoRepository {
    pub async fn get_by_report(&self, report_id: u64) -> Result<Vec<ReportPhoto>> {
        let query =
            SelectQuery::new(TABLE).with_condition("report_id", Value::from(report_id as i64));
        self.query_multiple(query).await
    }

    async fn query_multiple(&self, query: SelectQuery) -> Result<Vec<ReportPhoto>> {
        let records = self.db.get_records(query).await?;
        records.iter().map(ReportPhoto::from_attributes).collect()
    }
}

impl Injectable for ReportPhotoRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}
