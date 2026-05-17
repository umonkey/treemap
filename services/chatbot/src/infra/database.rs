use libsql::{params_from_iter, Builder, Connection, Database, Value};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct DatabaseClient {
    db: Database,
}

#[allow(dead_code)]
impl DatabaseClient {
    pub async fn new(path: &str) -> anyhow::Result<Self> {
        log::info!("Using database: {}", path);
        let db = Builder::new_local(path).build().await?;
        Ok(Self { db })
    }

    async fn connect(&self) -> anyhow::Result<Connection> {
        let conn = self.db.connect()?;
        Ok(conn)
    }

    pub async fn fetch_rows(
        &self,
        sql: &str,
        params: Vec<Value>,
    ) -> anyhow::Result<Vec<HashMap<String, Value>>> {
        let conn = self.connect().await?;
        let mut stmt = conn.prepare(sql).await?;
        let mut rows = stmt.query(params_from_iter(params)).await?;
        let mut result = Vec::new();

        let column_count = rows.column_count();
        let mut column_names = Vec::new();
        for i in 0..column_count {
            if let Some(name) = rows.column_name(i) {
                column_names.push(name.to_string());
            }
        }

        while let Some(row) = rows.next().await? {
            let mut map = HashMap::new();
            for (i, name) in column_names.iter().enumerate() {
                let value = row.get_value(i as i32)?;
                map.insert(name.clone(), value);
            }
            result.push(map);
        }

        Ok(result)
    }

    pub async fn exec(&self, sql: &str, params: Vec<Value>) -> anyhow::Result<i64> {
        let conn = self.connect().await?;
        let rows_affected = conn.execute(sql, params_from_iter(params)).await?;
        Ok(rows_affected as i64)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create_report(
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
        let conn = self.connect().await?;

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

    pub async fn count_trees(&self) -> anyhow::Result<i64> {
        let conn = self.connect().await?;
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM trees").await?;
        let mut rows = stmt.query(params_from_iter(Vec::<Value>::new())).await?;

        if let Some(row) = rows.next().await? {
            let count: i64 = row.get(0)?;
            Ok(count)
        } else {
            Ok(0)
        }
    }
}
