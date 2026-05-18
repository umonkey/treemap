use super::models::Report;
use crate::infra::database::{Database, SelectQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::get_timestamp;
use std::sync::Arc;

const TABLE: &str = "chatbot_reports";
const ONE_WEEK: u64 = 7 * 24 * 60 * 60;

pub struct ReportRepository {
    db: Arc<Database>,
}

impl ReportRepository {
    pub async fn get(&self, id: u64) -> Result<Option<Report>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));
        self.query_single(query).await
    }

    pub async fn get_active(&self) -> Result<Vec<Report>> {
        let now = get_timestamp();
        let since = now.saturating_sub(ONE_WEEK);

        // We need to filter by created_at > since AND lat IS NOT NULL AND lon IS NOT NULL.
        // SelectQuery doesn't seem to support IS NOT NULL directly easily with with_condition.
        // I'll use raw SQL or check if I can extend SelectQuery.
        // Looking at tree repository, they use fetch_sql for more complex queries.

        let sql = format!(
            "SELECT * FROM `{}` WHERE created_at >= ? AND lat IS NOT NULL AND lon IS NOT NULL",
            TABLE
        );
        let params = &[Value::from(since as i64)];

        self.fetch(&sql, params).await
    }

    async fn query_single(&self, query: SelectQuery) -> Result<Option<Report>> {
        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(Report::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn fetch(&self, sql: &str, params: &[Value]) -> Result<Vec<Report>> {
        let rows = self.db.fetch_sql(sql, params).await?;
        rows.iter().map(Report::from_attributes).collect()
    }
}

impl Injectable for ReportRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}
