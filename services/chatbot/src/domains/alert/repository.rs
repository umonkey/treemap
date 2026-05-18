use super::model::Alert;
use crate::infra::database::DatabaseClient;
use libsql::{params_from_iter, Value};
use std::sync::Arc;

pub struct AlertRepository {
    db: Arc<DatabaseClient>,
}

impl AlertRepository {
    pub fn new(db: Arc<DatabaseClient>) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: i64) -> anyhow::Result<Option<Alert>> {
        let conn = self.db.connect().await?;
        let sql = "SELECT id, created_at, created_by, chat_id, message_id, username, language_code, lat, lon, description, status, response_text, responded_at FROM chatbot_alerts WHERE id = ?";
        let mut stmt = conn.prepare(sql).await?;
        let mut rows = stmt
            .query(params_from_iter(vec![Value::Integer(id)]))
            .await?;

        if let Some(row) = rows.next().await? {
            Ok(Some(Alert {
                id: row.get(0)?,
                created_at: row.get(1)?,
                created_by: row.get(2)?,
                chat_id: row.get(3)?,
                message_id: row.get(4)?,
                username: row.get(5)?,
                language_code: row.get(6)?,
                lat: row.get(7)?,
                lon: row.get(8)?,
                description: row.get(9)?,
                status: row.get(10)?,
                response_text: row.get(11)?,
                responded_at: row.get(12)?,
            }))
        } else {
            Ok(None)
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        user_id: i64,
        chat_id: i64,
        message_id: Option<i32>,
        username: Option<String>,
        lang: Option<String>,
        lat: Option<f64>,
        lon: Option<f64>,
        force_new: bool,
    ) -> anyhow::Result<Alert> {
        let conn = self.db.connect().await?;

        if !force_new {
            let sql = "SELECT id FROM chatbot_alerts 
                       WHERE created_by = ? AND created_at > (unixepoch() - 600) 
                       ORDER BY created_at DESC LIMIT 1";
            let mut stmt = conn.prepare(sql).await?;
            let mut rows = stmt
                .query(params_from_iter(vec![Value::Integer(user_id)]))
                .await?;

            if let Some(row) = rows.next().await? {
                let id: i64 = row.get(0)?;
                return self
                    .get_by_id(id)
                    .await?
                    .ok_or_else(|| anyhow::anyhow!("Alert not found after discovery"));
            }
        }

        let sql = "INSERT INTO chatbot_alerts (created_at, created_by, chat_id, message_id, username, language_code, lat, lon) 
                   VALUES (unixepoch(), ?, ?, ?, ?, ?, ?, ?)";

        let params = vec![
            Value::Integer(user_id),
            Value::Integer(chat_id),
            message_id
                .map(|v| Value::Integer(v as i64))
                .unwrap_or(Value::Null),
            username.map(Value::Text).unwrap_or(Value::Null),
            lang.map(Value::Text).unwrap_or(Value::Null),
            lat.map(Value::Real).unwrap_or(Value::Null),
            lon.map(Value::Real).unwrap_or(Value::Null),
        ];

        conn.execute(sql, params_from_iter(params)).await?;
        let id = conn.last_insert_rowid();

        self.get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Alert not found after creation"))
    }

    pub async fn update_location(&self, id: i64, lat: f64, lon: f64) -> anyhow::Result<()> {
        let conn = self.db.connect().await?;
        let sql = "UPDATE chatbot_alerts SET lat = ?, lon = ? WHERE id = ?";
        let params = vec![Value::Real(lat), Value::Real(lon), Value::Integer(id)];
        conn.execute(sql, params_from_iter(params)).await?;
        Ok(())
    }

    pub async fn update_description(&self, id: i64, description: &str) -> anyhow::Result<()> {
        let conn = self.db.connect().await?;
        let sql = "UPDATE chatbot_alerts SET description = ? WHERE id = ?";
        let params = vec![Value::Text(description.to_string()), Value::Integer(id)];
        conn.execute(sql, params_from_iter(params)).await?;
        Ok(())
    }

    pub async fn get_active_id_by_user_id(&self, user_id: i64) -> anyhow::Result<Option<i64>> {
        let conn = self.db.connect().await?;
        let sql = "SELECT id FROM chatbot_alerts 
                   WHERE created_by = ? AND created_at > (unixepoch() - 600) 
                   ORDER BY created_at DESC LIMIT 1";
        let mut stmt = conn.prepare(sql).await?;
        let mut rows = stmt
            .query(params_from_iter(vec![Value::Integer(user_id)]))
            .await?;

        if let Some(row) = rows.next().await? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }
}
