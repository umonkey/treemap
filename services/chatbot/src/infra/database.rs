use libsql::{params_from_iter, Builder, Connection, Database, Value};
use std::collections::HashMap;
use std::time::Duration;

// WAL is persistent in the database header, so the first attempt normally
// succeeds. Retrying only covers the transient lock when another process is
// activating WAL at the same moment.
const WAL_ATTEMPTS: u32 = 3;
const WAL_INITIAL_RETRY_DELAY: Duration = Duration::from_millis(200);

#[allow(dead_code)]
pub struct DatabaseClient {
    db: Database,
}

#[allow(dead_code)]
impl DatabaseClient {
    pub async fn new(path: &str) -> anyhow::Result<Self> {
        log::info!("Using database: {}", path);
        let db = Builder::new_local(path).build().await?;
        let conn = db.connect()?;
        conn.execute_batch(
            "PRAGMA busy_timeout = 5000;
             PRAGMA synchronous = NORMAL;",
        )
        .await?;
        Self::enable_wal(&conn).await?;

        Ok(Self { db })
    }

    /// Switches the database to WAL with a bounded retry.
    ///
    /// `busy_timeout` must already be set on `conn`, otherwise a concurrent
    /// writer fails the switch immediately with `database is locked`.
    async fn enable_wal(conn: &Connection) -> anyhow::Result<()> {
        let mut attempt = 1;
        let mut delay = WAL_INITIAL_RETRY_DELAY;

        loop {
            match conn.execute_batch("PRAGMA journal_mode = WAL;").await {
                Ok(_) => return Ok(()),
                Err(error) if attempt < WAL_ATTEMPTS => {
                    log::warn!(
                        "Failed to set journal_mode = WAL (attempt {}/{}): {}. Retrying in {:?}",
                        attempt,
                        WAL_ATTEMPTS,
                        error,
                        delay
                    );
                    tokio::time::sleep(delay).await;
                    delay *= 2;
                    attempt += 1;
                }
                Err(error) => {
                    return Err(anyhow::anyhow!(
                        "Error setting database pragmas: journal_mode = WAL failed after {} attempts: {}",
                        WAL_ATTEMPTS,
                        error
                    ));
                }
            }
        }
    }

    pub async fn connect(&self) -> anyhow::Result<Connection> {
        let conn = self.db.connect()?;
        conn.execute_batch(
            "PRAGMA busy_timeout = 5000;
             PRAGMA synchronous = NORMAL;",
        )
        .await?;

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
}
