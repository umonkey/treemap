use super::models::Alert;
use crate::infra::database::{Database, SelectQuery, UpdateQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::get_timestamp;
use std::sync::Arc;

const TABLE: &str = "chatbot_alerts";

pub struct AlertRepository {
    db: Arc<Database>,
}

impl AlertRepository {
    pub async fn get(&self, id: u64) -> Result<Option<Alert>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));
        self.query_single(query).await
    }

    pub async fn create(
        &self,
        user_id: u64,
        lat: f64,
        lon: f64,
        description: &str,
    ) -> Result<Alert> {
        let sql = format!(
            "INSERT INTO `{}` (created_at, created_by, lat, lon, description, status) VALUES (?, ?, ?, ?, ?, 'new') RETURNING *",
            TABLE
        );

        let params = &[
            Value::from(get_timestamp() as i64),
            Value::from(user_id as i64),
            Value::from(lat),
            Value::from(lon),
            Value::from(description),
        ];

        let rows = self.db.fetch_sql(&sql, params).await?;

        let row = rows.first().ok_or_else(|| {
            Error::DatabaseStructure("Could not create an alert row.".to_string())
        })?;

        Alert::from_attributes(row)
    }

    pub async fn update_description(&self, id: u64, description: &str) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(id as i64))
            .with_value("description", Value::from(description));

        self.db.update(query).await?;

        Ok(())
    }

    pub async fn get_active(&self, days: u64) -> Result<Vec<Alert>> {
        let now = get_timestamp();
        let since = now.saturating_sub(days * 24 * 60 * 60);

        let sql = format!(
            "SELECT * FROM `{}` WHERE created_at >= ? AND lat IS NOT NULL AND lon IS NOT NULL",
            TABLE
        );
        let params = &[Value::from(since as i64)];

        self.fetch(&sql, params).await
    }

    pub async fn get_recent(&self, limit: u64) -> Result<Vec<Alert>> {
        let sql = format!(
            "SELECT * FROM `{}` ORDER BY created_at DESC, id DESC LIMIT ?",
            TABLE
        );

        self.fetch(&sql, &[Value::from(limit as i64)]).await
    }

    pub async fn get_filtered(
        &self,
        since_id: Option<u64>,
        status: Option<&str>,
        limit: u64,
        order_asc: bool,
    ) -> Result<Vec<Alert>> {
        let order_str = if order_asc { "ASC" } else { "DESC" };

        let sql = format!(
            "SELECT * FROM `{}` WHERE (? IS NULL OR id > ?) AND (? IS NULL OR status = ?) ORDER BY id {} LIMIT ?",
            TABLE, order_str
        );

        let params = vec![
            match since_id {
                Some(id) => Value::from(id as i64),
                None => Value::Null,
            },
            match since_id {
                Some(id) => Value::from(id as i64),
                None => Value::Null,
            },
            match status {
                Some(s) => Value::from(s.to_string()),
                None => Value::Null,
            },
            match status {
                Some(s) => Value::from(s.to_string()),
                None => Value::Null,
            },
            Value::from(limit as i64),
        ];

        self.fetch(&sql, &params).await
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
