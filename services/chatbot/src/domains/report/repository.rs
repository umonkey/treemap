use crate::infra::database::DatabaseClient;
use libsql::{params_from_iter, Value};
use std::sync::Arc;

pub struct ReportRepository {
    db: Arc<DatabaseClient>,
}

impl ReportRepository {
    pub fn new(db: Arc<DatabaseClient>) -> Self {
        Self { db }
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
    ) -> anyhow::Result<i64> {
        let conn = self.db.connect().await?;

        if !force_new {
            let sql = "SELECT id FROM chatbot_reports 
                       WHERE created_by = ? AND created_at > (unixepoch() - 600) 
                       ORDER BY created_at DESC LIMIT 1";
            let mut stmt = conn.prepare(sql).await?;
            let mut rows = stmt
                .query(params_from_iter(vec![Value::Integer(user_id)]))
                .await?;

            if let Some(row) = rows.next().await? {
                let id: i64 = row.get(0)?;
                return Ok(id);
            }
        }

        let sql = "INSERT INTO chatbot_reports (created_at, created_by, chat_id, message_id, username, language_code, lat, lon) 
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
        Ok(conn.last_insert_rowid())
    }
}
