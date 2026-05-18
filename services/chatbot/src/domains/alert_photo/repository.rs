use crate::infra::database::DatabaseClient;
use libsql::{params_from_iter, Value};
use std::sync::Arc;

pub struct AlertPhotoRepository {
    db: Arc<DatabaseClient>,
}

impl AlertPhotoRepository {
    pub fn new(db: Arc<DatabaseClient>) -> Self {
        Self { db }
    }

    pub async fn add_to_alert(&self, alert_id: i64, photo_path: &str) -> anyhow::Result<bool> {
        let conn = self.db.connect().await?;
        let sql = "INSERT INTO chatbot_alerts_photos (alert_id, photo_path) VALUES (?, ?)";
        let params = vec![
            Value::Integer(alert_id),
            Value::Text(photo_path.to_string()),
        ];
        conn.execute(sql, params_from_iter(params)).await?;
        let my_id = conn.last_insert_rowid();

        let first_id_sql =
            "SELECT id FROM chatbot_alerts_photos WHERE alert_id = ? ORDER BY id ASC LIMIT 1";
        let mut stmt = conn.prepare(first_id_sql).await?;
        let mut rows = stmt
            .query(params_from_iter(vec![Value::Integer(alert_id)]))
            .await?;

        if let Some(row) = rows.next().await? {
            let first_id: i64 = row.get(0)?;
            return Ok(my_id == first_id);
        }

        Ok(false)
    }

    pub async fn count_by_alert_id(&self, alert_id: i64) -> anyhow::Result<i64> {
        let conn = self.db.connect().await?;
        let sql = "SELECT COUNT(*) FROM chatbot_alerts_photos WHERE alert_id = ?";
        let mut stmt = conn.prepare(sql).await?;
        let mut rows = stmt
            .query(params_from_iter(vec![Value::Integer(alert_id)]))
            .await?;

        if let Some(row) = rows.next().await? {
            Ok(row.get(0)?)
        } else {
            Ok(0)
        }
    }
}
