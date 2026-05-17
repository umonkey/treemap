use crate::infra::database::DatabaseClient;
use libsql::{params_from_iter, Value};
use std::sync::Arc;

pub struct PhotoRepository {
    db: Arc<DatabaseClient>,
}

impl PhotoRepository {
    pub fn new(db: Arc<DatabaseClient>) -> Self {
        Self { db }
    }

    pub async fn add_to_report(&self, report_id: i64, photo_id: &str) -> anyhow::Result<bool> {
        let conn = self.db.connect().await?;
        let sql = "INSERT INTO chatbot_report_photos (report_id, photo_id) VALUES (?, ?)";
        let params = vec![Value::Integer(report_id), Value::Text(photo_id.to_string())];
        conn.execute(sql, params_from_iter(params)).await?;
        let my_id = conn.last_insert_rowid();

        let first_id_sql =
            "SELECT id FROM chatbot_report_photos WHERE report_id = ? ORDER BY id ASC LIMIT 1";
        let mut stmt = conn.prepare(first_id_sql).await?;
        let mut rows = stmt
            .query(params_from_iter(vec![Value::Integer(report_id)]))
            .await?;

        if let Some(row) = rows.next().await? {
            let first_id: i64 = row.get(0)?;
            return Ok(my_id == first_id);
        }

        Ok(false)
    }
}
