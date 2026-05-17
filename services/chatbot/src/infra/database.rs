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
