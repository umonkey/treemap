use super::models::Alert;
use crate::infra::database::{Database, SelectQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::get_timestamp;
use std::sync::Arc;

const TABLE: &str = "chatbot_alerts";
const ONE_WEEK: u64 = 7 * 24 * 60 * 60;

pub struct AlertRepository {
    db: Arc<Database>,
}

impl AlertRepository {
    pub async fn get(&self, id: u64) -> Result<Option<Alert>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));
        self.query_single(query).await
    }

    pub async fn get_active(&self) -> Result<Vec<Alert>> {
        let now = get_timestamp();
        let since = now.saturating_sub(ONE_WEEK);

        let sql = format!(
            "SELECT * FROM `{}` WHERE created_at >= ? AND lat IS NOT NULL AND lon IS NOT NULL",
            TABLE
        );
        let params = &[Value::from(since as i64)];

        self.fetch(&sql, params).await
    }

    async fn query_single(&self, query: SelectQuery) -> Result<Option<Alert>> {
        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(Alert::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn fetch(&self, sql: &str, params: &[Value]) -> Result<Vec<Alert>> {
        let rows = self.db.fetch_sql(sql, params).await?;
        rows.iter().map(Alert::from_attributes).collect()
    }
}

impl Injectable for AlertRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}
