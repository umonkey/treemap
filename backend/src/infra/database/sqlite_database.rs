//! Database client implementation for Turso.
//!
//! @docs https://docs.turso.tech/sdk/rust/quickstart

use super::base::DatabaseInterface;
use super::queries::*;
use crate::domain::tree::Tree;
use crate::infra::database::{Attributes, Value};
use crate::types::*;
use crate::utils::get_timestamp;
use crate::utils::get_unique_id;
use async_trait::async_trait;
use libsql::params_from_iter;
use libsql::{Builder, Database, Transaction};
use log::{debug, error, info};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

const SUGGEST_WINDOW: u64 = 3600 * 24; // 24 hours

// This is where temporary database files are created to simulate
// the in-memory database, which doesn't really work as we need with libsql.
const TEMP_DB_DIR: &str = "var/test-files";

pub struct SqliteDatabase {
    pool: Arc<Database>,
}

pub struct SqliteTransaction {
    tx: Arc<Mutex<Option<Transaction>>>,
}

impl SqliteTransaction {
    pub fn new(tx: Transaction) -> Self {
        Self {
            tx: Arc::new(Mutex::new(Some(tx))),
        }
    }

    async fn fetch(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>> {
        let start = Instant::now();
        let mut tx_lock = self.tx.lock().await;
        let tx = tx_lock
            .as_mut()
            .ok_or_else(|| Error::DatabaseQuery("Transaction already closed".to_string()))?;

        let mut stmt = tx.prepare(query).await.inspect_err(|e| {
            error!("Error preparing SQL statement: {e}");
        })?;

        let mut rows = stmt
            .query(params_from_iter(params))
            .await
            .inspect_err(|e| {
                error!("Error executing SQL statement: {e}");
            })?;

        let mut res: Vec<Attributes> = Vec::new();

        while let Some(row) = rows.next().await? {
            res.push(row.into());
        }

        let duration = start.elapsed();

        info!(
            "Read {} rows in {}ms for query: {query}",
            res.len(),
            duration.as_millis()
        );

        Ok(res)
    }
}

impl SqliteDatabase {
    // Creates a client for a remote database.
    pub async fn from_remote(url: &str, token: &str) -> Result<Self> {
        let pool = Builder::new_remote(url.to_string(), token.to_string())
            .build()
            .await
            .map_err(|e| {
                error!("Error connecting to a Turso database at {url}: {e}");
                Error::DatabaseConnect
            })?;

        info!("Connected to a Turso database at {url}");

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    // Creates a client for a local database.
    pub async fn from_local(path: &str) -> Result<Self> {
        let pool = Builder::new_local(path).build().await.map_err(|e| {
            error!("Error opening an SQLite database in {path}: {e}");
            Error::DatabaseConnect
        })?;

        info!("Using an SQLite database in {path}");

        Self::setup_pool(&pool).await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    // Creates a client for a local database.
    // NB: this creates a new database per connection.
    pub async fn from_memory() -> Result<Self> {
        let path = Self::get_temporary_path();

        let pool = Builder::new_local(&path).build().await.map_err(|e| {
            error!("Error opening an in-memory SQLite database: {e}");
            Error::DatabaseConnect
        })?;

        info!("Created a temporary SQLite database in {path}.");

        Self::setup_pool(&pool).await?;

        info!("Database schema initialized.");

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub async fn execute_batch(&self, script: &str) -> Result<()> {
        Self::execute_pool(&self.pool, script.to_string()).await
    }

    async fn execute_pool(pool: &Database, script: String) -> Result<()> {
        let conn = pool.connect().inspect_err(|e| {
            error!("Error connecting to SQLite: {e}");
        })?;

        conn.execute_batch(&script).await.inspect_err(|e| {
            error!("Error executing SQL script: {e}");
        })?;

        Ok(())
    }

    fn get_temporary_path() -> String {
        if !std::path::Path::new(TEMP_DB_DIR).is_dir() {
            if let Err(e) = std::fs::create_dir_all(TEMP_DB_DIR) {
                error!("Could not create temporary db location {TEMP_DB_DIR}: {e}");
            }
        }

        let id = get_unique_id().unwrap_or(0);
        format!("{TEMP_DB_DIR}/db-{id}.db")
    }

    // Configure the SQLite engine.
    // Note that this does not work for remote connections.
    async fn setup_pool(pool: &Database) -> Result<()> {
        let conn = pool.connect()?;

        conn.execute_batch("PRAGMA journal_mode = WAL;")
            .await
            .inspect_err(|e| {
                error!("Error setting journal_mode: {e}");
            })?;

        conn.execute_batch("PRAGMA synchronous = NORMAL;")
            .await
            .inspect_err(|e| {
                error!("Error setting synchronous: {e}");
            })?;

        if cfg!(test) {
            let schema = include_str!("../../../dev/schema-sqlite.sql");
            conn.execute_batch(schema).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl DatabaseInterface for SqliteTransaction {
    async fn transact(&self) -> Result<Box<dyn DatabaseInterface>> {
        Err(Error::DatabaseQuery("Already in a transaction".to_string()))
    }

    async fn commit(&self) -> Result<()> {
        let mut tx_lock = self.tx.lock().await;
        let tx = tx_lock
            .take()
            .ok_or_else(|| Error::DatabaseQuery("Transaction already closed".to_string()))?;
        tx.commit()
            .await
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?;
        Ok(())
    }

    async fn rollback(&self) -> Result<()> {
        let mut tx_lock = self.tx.lock().await;
        let tx = tx_lock
            .take()
            .ok_or_else(|| Error::DatabaseQuery("Transaction already closed".to_string()))?;
        tx.rollback()
            .await
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?;
        Ok(())
    }

    async fn sql(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>> {
        self.fetch(query, params).await
    }

    async fn execute_sql(&self, query: &str, params: &[Value]) -> Result<()> {
        let mut tx_lock = self.tx.lock().await;
        let tx = tx_lock
            .as_mut()
            .ok_or_else(|| Error::DatabaseQuery("Transaction already closed".to_string()))?;
        tx.execute(query, params_from_iter(params.to_vec()))
            .await
            .inspect_err(|e| {
                error!("Error executing SQL statement: {e}");
            })?;
        Ok(())
    }

    async fn execute(&self, _query: &str) -> Result<()> {
        Err(Error::DatabaseQuery(
            "Batch execution is not supported in transactions".to_string(),
        ))
    }

    async fn get_record(&self, query: SelectQuery) -> Result<Option<Attributes>> {
        let query = query.with_limit(1);
        let records = self.get_records(query).await?;
        Ok(records.first().cloned())
    }

    async fn get_records(&self, query: SelectQuery) -> Result<Vec<Attributes>> {
        let (sql, params) = query.build();

        self.sql(sql.as_str(), params.as_slice())
            .await
            .inspect_err(|e| {
                debug!("SQL query failed: {e}; SQL={sql}");
            })
    }

    async fn add_record(&self, query: InsertQuery) -> Result<()> {
        let (sql, params) = query.build();
        self.execute_sql(sql.as_str(), params.as_slice()).await
    }

    async fn replace(&self, query: ReplaceQuery) -> Result<()> {
        let (sql, params) = query.build();
        self.execute_sql(sql.as_str(), params.as_slice()).await
    }

    async fn update(&self, query: UpdateQuery) -> Result<u64> {
        let mut tx_lock = self.tx.lock().await;
        let tx = tx_lock
            .as_mut()
            .ok_or_else(|| Error::DatabaseQuery("Transaction already closed".to_string()))?;

        let (sql, params) = query.build();

        let rows = tx
            .execute(sql.as_str(), params_from_iter(params))
            .await
            .inspect_err(|e| {
                error!("Error updating database: {e}");
            })?;

        Ok(rows)
    }

    async fn delete(&self, query: DeleteQuery) -> Result<u64> {
        let mut tx_lock = self.tx.lock().await;
        let tx = tx_lock
            .as_mut()
            .ok_or_else(|| Error::DatabaseQuery("Transaction already closed".to_string()))?;

        let (sql, params) = query.build();

        let rows = tx
            .execute(sql.as_str(), params_from_iter(params))
            .await
            .inspect_err(|e| {
                error!("Error deleting record: {e}");
            })?;

        Ok(rows)
    }

    async fn increment(&self, query: IncrementQuery) -> Result<()> {
        let (sql, params) = query.build();
        self.execute_sql(sql.as_str(), params.as_slice()).await
    }

    async fn count(&self, query: CountQuery) -> Result<u64> {
        let (sql, params) = query.build();

        let rows = self.fetch(&sql, &params).await?;

        if let Some(value) = rows.first() {
            return value.require_u64("count");
        }

        Ok(0)
    }

    async fn find_streets(&self, query: &str) -> Result<Vec<String>> {
        let pattern = format!("%{}%", query.trim().to_lowercase());

        let rows = self.fetch(
            "SELECT DISTINCT address FROM trees WHERE state <> 'gone' AND address LIKE ?1 ORDER BY address LIMIT 10",
            &[Value::from(pattern)],
        ).await?;

        let mut streets: Vec<String> = Vec::new();

        for row in rows {
            streets.push(row.require_string("address")?);
        }

        Ok(streets)
    }

    async fn find_recent_species(&self, user_id: u64) -> Result<Vec<String>> {
        let since = get_timestamp() - SUGGEST_WINDOW;

        let rows = self.fetch(
            "SELECT species, COUNT(1) AS use_count FROM trees WHERE updated_by = ? AND updated_at >= ? AND LOWER(species) <> 'unknown' GROUP BY species ORDER BY use_count DESC LIMIT 10",
            &[Value::from(user_id), Value::from(since)],
        ).await?;

        let mut values: Vec<String> = Vec::new();

        for row in rows {
            values.push(row.require_string("species")?);
        }

        Ok(values)
    }

    async fn get_species_stats(&self) -> Result<Vec<(String, u64)>> {
        let rows = self.fetch(
            "SELECT species, COUNT(1) AS cnt FROM trees WHERE state <> 'gone' AND state <> 'stump' GROUP BY TRIM(LOWER(species)) ORDER BY cnt DESC, LOWER(species)",
            &[],
        ).await?;

        let mut res = Vec::new();

        for row in rows {
            let species = row.require_string("species")?;
            let count = row.require_u64("cnt")?;
            res.push((species, count));
        }

        Ok(res)
    }

    async fn get_species_mismatch(&self, count: u64, skip: u64) -> Result<Vec<Tree>> {
        let rows = self.fetch(
            "SELECT id, osm_id, lat, lon, species, notes, height, circumference, diameter, state, added_at, updated_at, added_by, thumbnail_id, year, address FROM trees WHERE state <> 'gone' AND species <> 'Unknown species' AND species <> 'Unknown' AND species NOT IN (SELECT name FROM species) LIMIT ? OFFSET ?",
            &[Value::from(count), Value::from(skip)],
        ).await?;

        let mut records = Vec::new();

        for row in rows {
            if let Ok(packed) = Tree::from_attributes(&row) {
                records.push(packed);
            }
        }

        Ok(records)
    }

    async fn get_top_streets(&self, count: u64) -> Result<Vec<(String, u64)>> {
        let rows = self.fetch(
            "SELECT address, COUNT(1) AS cnt FROM trees WHERE state <> 'gone' AND address IS NOT NULL GROUP BY LOWER(address) ORDER BY cnt DESC, LOWER(address) LIMIT ?",
            &[Value::from(count)],
        ).await?;

        let mut res = Vec::new();

        for row in rows {
            let address = row.require_string("address")?;
            let count = row.require_u64("cnt")?;
            res.push((address, count));
        }

        Ok(res)
    }

    async fn get_state_stats(&self) -> Result<Vec<(String, u64)>> {
        let rows = self.fetch(
            "SELECT state, COUNT(1) AS cnt FROM trees WHERE state IS NOT NULL GROUP BY state ORDER BY cnt DESC",
            &[],
        ).await?;

        let mut res = Vec::new();

        for row in rows {
            let state = row.require_string("state")?;
            let count = row.require_u64("cnt")?;
            res.push((state, count));
        }

        Ok(res)
    }

    async fn get_heatmap(&self, after: u64, before: u64) -> Result<Vec<(String, u64)>> {
        let rows = self.fetch(
            "SELECT DATE(added_at, 'unixepoch') AS date, COUNT(distinct tree_id) AS count FROM trees_props WHERE added_at >= ? AND added_at < ? GROUP BY date ORDER BY date",
            &[Value::from(after), Value::from(before)],
        ).await?;

        let mut heatmap = Vec::new();

        for row in rows {
            let date = row.require_string("date")?;
            let count = row.require_u64("count")?;
            heatmap.push((date, count));
        }

        Ok(heatmap)
    }

    async fn get_user_heatmap(
        &self,
        after: u64,
        before: u64,
        user_id: u64,
    ) -> Result<Vec<(String, u64)>> {
        let rows = self.fetch(
            "SELECT DATE(added_at, 'unixepoch') AS date, COUNT(distinct tree_id) AS count FROM trees_props WHERE added_at >= ? AND added_at < ? AND added_by = ? GROUP BY date ORDER BY date",
            &[
                Value::from(after),
                Value::from(before),
                Value::from(user_id),
            ],
        ).await?;

        let mut heatmap = Vec::new();

        for row in rows {
            let date = row.require_string("date")?;
            let count = row.require_u64("count")?;
            heatmap.push((date, count));
        }

        Ok(heatmap)
    }
}

#[async_trait]
impl DatabaseInterface for SqliteDatabase {
    async fn transact(&self) -> Result<Box<dyn DatabaseInterface>> {
        let conn = self.pool.connect()?;
        let tx = conn
            .transaction()
            .await
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?;
        Ok(Box::new(SqliteTransaction::new(tx)))
    }

    async fn commit(&self) -> Result<()> {
        Ok(())
    }

    async fn rollback(&self) -> Result<()> {
        Ok(())
    }

    async fn sql(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>> {
        self.transact().await?.sql(query, params).await
    }

    async fn execute_sql(&self, query: &str, params: &[Value]) -> Result<()> {
        let tx = self.transact().await?;
        tx.execute_sql(query, params).await?;
        tx.commit().await
    }

    async fn execute(&self, query: &str) -> Result<()> {
        self.execute_batch(query).await
    }

    async fn get_record(&self, query: SelectQuery) -> Result<Option<Attributes>> {
        self.transact().await?.get_record(query).await
    }

    async fn get_records(&self, query: SelectQuery) -> Result<Vec<Attributes>> {
        self.transact().await?.get_records(query).await
    }

    async fn add_record(&self, query: InsertQuery) -> Result<()> {
        let tx = self.transact().await?;
        tx.add_record(query).await?;
        tx.commit().await
    }

    async fn replace(&self, query: ReplaceQuery) -> Result<()> {
        let tx = self.transact().await?;
        tx.replace(query).await?;
        tx.commit().await
    }

    async fn update(&self, query: UpdateQuery) -> Result<u64> {
        let tx = self.transact().await?;
        let res = tx.update(query).await?;
        tx.commit().await?;
        Ok(res)
    }

    async fn delete(&self, query: DeleteQuery) -> Result<u64> {
        let tx = self.transact().await?;
        let res = tx.delete(query).await?;
        tx.commit().await?;
        Ok(res)
    }

    async fn increment(&self, query: IncrementQuery) -> Result<()> {
        let tx = self.transact().await?;
        tx.increment(query).await?;
        tx.commit().await
    }

    async fn count(&self, query: CountQuery) -> Result<u64> {
        self.transact().await?.count(query).await
    }

    async fn find_streets(&self, query: &str) -> Result<Vec<String>> {
        self.transact().await?.find_streets(query).await
    }

    async fn find_recent_species(&self, user_id: u64) -> Result<Vec<String>> {
        self.transact().await?.find_recent_species(user_id).await
    }

    async fn get_species_stats(&self) -> Result<Vec<(String, u64)>> {
        self.transact().await?.get_species_stats().await
    }

    async fn get_species_mismatch(&self, count: u64, skip: u64) -> Result<Vec<Tree>> {
        self.transact()
            .await?
            .get_species_mismatch(count, skip)
            .await
    }

    async fn get_top_streets(&self, count: u64) -> Result<Vec<(String, u64)>> {
        self.transact().await?.get_top_streets(count).await
    }

    async fn get_state_stats(&self) -> Result<Vec<(String, u64)>> {
        self.transact().await?.get_state_stats().await
    }

    async fn get_heatmap(&self, after: u64, before: u64) -> Result<Vec<(String, u64)>> {
        self.transact().await?.get_heatmap(after, before).await
    }

    async fn get_user_heatmap(
        &self,
        after: u64,
        before: u64,
        user_id: u64,
    ) -> Result<Vec<(String, u64)>> {
        self.transact()
            .await?
            .get_user_heatmap(after, before, user_id)
            .await
    }
}

impl Clone for SqliteDatabase {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;

    async fn setup() -> SqliteDatabase {
        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        let db = SqliteDatabase::from_memory()
            .await
            .expect("Error creating database.");

        db.execute(include_str!("./fixtures/init.sql"))
            .await
            .expect("Error installing fixtures.");

        info!("SQLite database initialized.");

        db
    }

    #[tokio::test]
    async fn test_species_stats() {
        let db = setup().await;

        db.execute(include_str!("./fixtures/test_species_stats.sql"))
            .await
            .expect("Error adding species.");

        let res = db.get_species_stats().await.expect("Error getting report.");

        assert_eq!(res.len(), 1, "Could not find species for oak.");
        assert_eq!("Quercus robur", res[0].0);
        assert_eq!(2, res[0].1);
    }

    #[tokio::test]
    async fn test_heatmap() {
        let db = setup().await;

        db.execute(include_str!("./fixtures/test_heatmap.sql"))
            .await
            .expect("Error adding heatmap input.");

        let res = db
            .get_heatmap(1754298465, 1754384866)
            .await
            .expect("Error getting heatmap.");

        assert_eq!(res.len(), 2, "Wrong number of heatmap entries.");
        assert_eq!("2025-08-04", res[0].0);
        assert_eq!(1, res[0].1);
        assert_eq!("2025-08-05", res[1].0);
        assert_eq!(2, res[1].1);
    }

    #[tokio::test]
    async fn test_user_heatmap() {
        let db = setup().await;

        db.execute(include_str!("./fixtures/test_heatmap.sql"))
            .await
            .expect("Error adding heatmap input.");

        let res = db
            .get_user_heatmap(1754298465, 1754384866, 1)
            .await
            .expect("Error getting heatmap.");

        assert_eq!(res.len(), 2, "Wrong number of heatmap entries.");
        assert_eq!("2025-08-04", res[0].0);
        assert_eq!(1, res[0].1);
        assert_eq!("2025-08-05", res[1].0);
        assert_eq!(1, res[1].1);
    }
}
