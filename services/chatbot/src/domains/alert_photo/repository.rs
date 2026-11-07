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

    /// Creates the universal `files` record and its `original` variant for a
    /// photo that has already been written to object storage.
    pub async fn create_file(
        &self,
        file_id: u64,
        created_by: i64,
        size_bytes: i64,
    ) -> anyhow::Result<()> {
        let conn = self.db.connect().await?;

        let sql = "INSERT INTO files (id, kind, status, is_visible, created_at, created_by, ready_at) VALUES (?, 'photo', 'ready', 1, unixepoch(), ?, unixepoch())";
        let params = vec![Value::Integer(file_id as i64), Value::Integer(created_by)];
        conn.execute(sql, params_from_iter(params)).await?;

        let key = format!("files/{}/original.jpg", file_id);
        let sql = "INSERT INTO files_variants (file_id, name, `key`, mime_type, size_bytes, width, height, created_at) VALUES (?, 'original', ?, 'image/jpeg', ?, NULL, NULL, unixepoch())";
        let params = vec![
            Value::Integer(file_id as i64),
            Value::Text(key),
            Value::Integer(size_bytes),
        ];
        conn.execute(sql, params_from_iter(params)).await?;

        Ok(())
    }

    pub async fn add_to_alert(
        &self,
        alert_id: i64,
        file_id: u64,
        added_by: i64,
    ) -> anyhow::Result<bool> {
        let conn = self.db.connect().await?;

        let sql = "INSERT OR IGNORE INTO chatbot_alerts_files (alert_id, file_id, added_at, added_by) VALUES (?, ?, unixepoch(), ?)";
        let params = vec![
            Value::Integer(alert_id),
            Value::Integer(file_id as i64),
            Value::Integer(added_by),
        ];
        conn.execute(sql, params_from_iter(params)).await?;

        let count = count_by_alert_id(&conn, alert_id).await?;

        Ok(count == 1)
    }

    pub async fn count_by_alert_id(&self, alert_id: i64) -> anyhow::Result<i64> {
        let conn = self.db.connect().await?;
        count_by_alert_id(&conn, alert_id).await
    }

    pub async fn get_photos_by_alert_id(&self, alert_id: i64) -> anyhow::Result<Vec<String>> {
        let conn = self.db.connect().await?;

        // Legacy rows predate the universal file storage, so they are returned
        // first, followed by the new `files_variants` keys in insertion order.
        let sql = "SELECT variant_key FROM (
                       SELECT photo_path AS variant_key, 0 AS source, id AS sort_id
                       FROM chatbot_alerts_photos
                       WHERE alert_id = ?
                       UNION ALL
                       SELECT v.`key` AS variant_key, 1 AS source, l.id AS sort_id
                       FROM chatbot_alerts_files l
                       JOIN files_variants v ON v.file_id = l.file_id AND v.name = 'original'
                       WHERE l.alert_id = ?
                   )
                   ORDER BY source ASC, sort_id ASC";
        let params = vec![Value::Integer(alert_id), Value::Integer(alert_id)];
        let mut stmt = conn.prepare(sql).await?;
        let mut rows = stmt.query(params_from_iter(params)).await?;

        let mut paths = Vec::new();
        while let Some(row) = rows.next().await? {
            paths.push(row.get(0)?);
        }

        Ok(paths)
    }
}

async fn count_by_alert_id(conn: &libsql::Connection, alert_id: i64) -> anyhow::Result<i64> {
    let sql = "SELECT (SELECT COUNT(*) FROM chatbot_alerts_files WHERE alert_id = ?)
                    + (SELECT COUNT(*) FROM chatbot_alerts_photos WHERE alert_id = ?)";
    let params = vec![Value::Integer(alert_id), Value::Integer(alert_id)];
    let mut stmt = conn.prepare(sql).await?;
    let mut rows = stmt.query(params_from_iter(params)).await?;

    if let Some(row) = rows.next().await? {
        Ok(row.get(0)?)
    } else {
        Ok(0)
    }
}
