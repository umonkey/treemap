use crate::infra::database::DatabaseClient;
use libsql::{params_from_iter, Value};
use std::sync::Arc;

pub struct TreeRepository {
    db: Arc<DatabaseClient>,
}

impl TreeRepository {
    pub fn new(db: Arc<DatabaseClient>) -> Self {
        Self { db }
    }

    pub async fn count(&self) -> anyhow::Result<i64> {
        let conn = self.db.connect().await?;
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
