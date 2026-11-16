//! Database client implementation for Turso.
//!
//! @docs https://docs.turso.tech/sdk/rust/quickstart

use super::base::DatabaseInterface;
use super::queries::*;
use crate::infra::database::{Attributes, Value};
use crate::types::*;
use crate::utils::get_unique_id;
use async_trait::async_trait;
use libsql::params_from_iter;
use libsql::{Builder, Connection, Database, Transaction, TransactionBehavior};
use log::{debug, info, warn};
use std::future::Future;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, MutexGuard};

// This is where temporary database files are created to simulate
// the in-memory database, which doesn't really work as we need with libsql.
const TEMP_DB_DIR: &str = "var/test-files";

// A statement that failed to acquire the write lock made no changes, so
// re-executing it is safe. Contention beyond busy_timeout is rare, so the
// attempt count stays small and only the non-transactional writes retry.
const BUSY_ATTEMPTS: u32 = 3;
const BUSY_INITIAL_DELAY: Duration = Duration::from_millis(100);

// Runs a write operation, retrying only when SQLite reports a transient lock.
// Any other error is returned on the first attempt. Retries are logged at
// debug: the final error is logged once at the response boundary.
async fn retry_on_busy<T, F, Fut>(mut operation: F) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut attempt = 1;

    loop {
        match operation().await {
            Ok(value) => return Ok(value),

            Err(error) if error.is_transient_lock() && attempt < BUSY_ATTEMPTS => {
                let delay = BUSY_INITIAL_DELAY * 2u32.pow(attempt - 1);

                debug!(
                    "Database is locked (attempt {attempt} of {BUSY_ATTEMPTS}): {error}; retrying in {}ms",
                    delay.as_millis()
                );

                tokio::time::sleep(delay).await;

                attempt += 1;
            }

            Err(error) => return Err(error),
        }
    }
}

pub struct SqliteDatabase {
    pool: Arc<Database>,
    // A remote database does not guarantee a non-deferred transaction, so
    // immediate transactions fall back to the deferred behaviour there.
    is_remote: bool,
}

pub struct SqliteTransaction {
    tx: Arc<Mutex<Option<Transaction>>>,
    _conn: Connection,
}

impl SqliteTransaction {
    pub fn new(tx: Transaction, conn: Connection) -> Self {
        Self {
            tx: Arc::new(Mutex::new(Some(tx))),
            _conn: conn,
        }
    }

    async fn lock(&self) -> Result<MutexGuard<'_, Option<Transaction>>> {
        let tx_lock = self.tx.lock().await;

        if tx_lock.is_none() {
            return Err(Error::DatabaseQuery(
                "Transaction already closed".to_string(),
            ));
        }

        Ok(tx_lock)
    }

    async fn take_tx(&self) -> Result<Transaction> {
        self.tx
            .lock()
            .await
            .take()
            .ok_or_else(|| Error::DatabaseQuery("Transaction already closed".to_string()))
    }

    async fn execute_sql_internal(&self, query: &str, params: &[Value]) -> Result<u64> {
        let mut tx_lock = self.lock().await?;
        let tx = tx_lock
            .as_mut()
            .expect("Invariant: the transaction must be open while the lock is held");

        let rows = tx
            .execute(query, params_from_iter(params.to_vec()))
            .await
            .map_err(Error::from)?;

        Ok(rows)
    }

    async fn fetch_sql(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>> {
        let start = Instant::now();
        let mut tx_lock = self.lock().await?;
        let tx = tx_lock
            .as_mut()
            .expect("Invariant: the transaction must be open while the lock is held");

        let mut stmt = tx.prepare(query).await.map_err(|e| {
            Error::DatabaseQuery(format!("Error preparing SQL statement: {e}; query={query}"))
        })?;

        let mut rows = stmt.query(params_from_iter(params)).await.map_err(|e| {
            Error::DatabaseQuery(format!("Error executing SQL statement: {e}; query={query}"))
        })?;

        let mut res: Vec<Attributes> = Vec::new();

        while let Some(row) = rows.next().await? {
            res.push(row.into());
        }

        let duration = start.elapsed();

        debug!(
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
                Error::DatabaseConnect(format!(
                    "Error connecting to a Turso database at {url}: {e}"
                ))
            })?;

        info!("Connected to a Turso database at {}", url);

        Ok(Self {
            pool: Arc::new(pool),
            is_remote: true,
        })
    }

    // Creates a client for a local database.
    pub async fn from_local(path: &str) -> Result<Self> {
        let pool = Builder::new_local(path).build().await.map_err(|e| {
            Error::DatabaseConnect(format!("Error opening an SQLite database in {path}: {e}"))
        })?;

        info!("Using an SQLite database in {}", path);

        Self::setup_pool(&pool).await?;

        Ok(Self {
            pool: Arc::new(pool),
            is_remote: false,
        })
    }

    // Creates a client for a local database.
    // NB: this creates a new database per connection.
    pub async fn from_memory() -> Result<Self> {
        let path = Self::get_temporary_path()?;

        let pool = Builder::new_local(&path).build().await.map_err(|e| {
            Error::DatabaseConnect(format!("Error opening an in-memory SQLite database: {e}"))
        })?;

        info!("Created a temporary SQLite database in {}.", path);

        Self::setup_pool(&pool).await?;

        info!("Database schema initialized.");

        Ok(Self {
            pool: Arc::new(pool),
            is_remote: false,
        })
    }

    fn get_temporary_path() -> Result<String> {
        if !std::path::Path::new(TEMP_DB_DIR).is_dir() {
            std::fs::create_dir_all(TEMP_DB_DIR).map_err(|e| {
                Error::DatabaseConnect(format!(
                    "Could not create temporary db location {TEMP_DB_DIR}: {e}"
                ))
            })?;
        }

        let id = get_unique_id()?;
        Ok(format!("{TEMP_DB_DIR}/db-{id}.db"))
    }

    // Configure the SQLite engine.
    // Note that this does not work for remote connections.
    async fn setup_pool(pool: &Database) -> Result<()> {
        let conn = pool
            .connect()
            .map_err(|e| Error::DatabaseConnect(e.to_string()))?;

        // The busy handler and durability are set before the journal mode
        // switch, because switching the journal mode may need a lock.
        conn.execute_batch(
            "PRAGMA busy_timeout = 5000;
             PRAGMA synchronous = NORMAL;",
        )
        .await
        .map_err(|e| Error::DatabaseQuery(format!("Error setting database pragmas: {e}")))?;

        Self::enable_wal(&conn).await?;

        if cfg!(test) {
            let schema = include_str!("../../../dev/schema-sqlite.sql");
            conn.execute_batch(schema).await?;
        }

        Ok(())
    }

    // Switches the database to WAL, retrying briefly: the switch needs a lock
    // that another process may hold while the backend starts. WAL is persisted
    // in the file header, so a successful first attempt is the normal path.
    async fn enable_wal(conn: &Connection) -> Result<()> {
        const MAX_ATTEMPTS: u32 = 3;

        let mut last_error = String::new();

        for attempt in 1..=MAX_ATTEMPTS {
            match conn.execute_batch("PRAGMA journal_mode = WAL;").await {
                Ok(_) => return Ok(()),

                Err(e) => {
                    last_error = e.to_string();

                    if attempt < MAX_ATTEMPTS {
                        let delay = Duration::from_millis(200 * 2u64.pow(attempt - 1));

                        warn!(
                            "Failed to enable the WAL journal mode (attempt {attempt} of {MAX_ATTEMPTS}): {e}; retrying in {}ms",
                            delay.as_millis()
                        );

                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }

        Err(Error::DatabaseQuery(format!(
            "Error setting database pragmas: journal_mode WAL failed after {MAX_ATTEMPTS} attempts: {last_error}"
        )))
    }

    async fn connect(&self) -> Result<Connection> {
        let conn = self
            .pool
            .connect()
            .map_err(|e| Error::DatabaseConnect(e.to_string()))?;

        // synchronous is per connection, so it has to be set on every one.
        conn.execute_batch(
            "PRAGMA busy_timeout = 5000;
             PRAGMA synchronous = NORMAL;",
        )
        .await
        .map_err(|e| Error::DatabaseQuery(format!("Error setting connection pragmas: {e}")))?;

        Ok(conn)
    }
}

#[async_trait]
impl DatabaseInterface for SqliteTransaction {
    async fn transact(&self) -> Result<Box<dyn DatabaseInterface>> {
        Err(Error::DatabaseQuery("Already in a transaction".to_string()))
    }

    async fn transact_immediate(&self) -> Result<Box<dyn DatabaseInterface>> {
        Err(Error::DatabaseQuery("Already in a transaction".to_string()))
    }

    async fn commit(&self) -> Result<()> {
        let tx = self.take_tx().await?;
        tx.commit()
            .await
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?;
        Ok(())
    }

    async fn rollback(&self) -> Result<()> {
        let tx = self.take_tx().await?;
        tx.rollback()
            .await
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?;
        Ok(())
    }

    async fn fetch_sql(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>> {
        SqliteTransaction::fetch_sql(self, query, params).await
    }

    async fn execute_sql(&self, query: &str, params: &[Value]) -> Result<u64> {
        self.execute_sql_internal(query, params).await
    }

    async fn execute_batch(&self, query: &str) -> Result<()> {
        let mut tx_lock = self.lock().await?;
        let tx = tx_lock
            .as_mut()
            .expect("Invariant: the transaction must be open while the lock is held");

        tx.execute_batch(query).await.map_err(|e| {
            Error::DatabaseQuery(format!("Error executing SQL batch: {e}; query={query}"))
        })?;

        Ok(())
    }

    async fn get_record(&self, query: SelectQuery) -> Result<Option<Attributes>> {
        let query = query.with_limit(1);
        let records = self.get_records(query).await?;
        Ok(records.first().cloned())
    }

    async fn get_records(&self, query: SelectQuery) -> Result<Vec<Attributes>> {
        let (sql, params) = query.build();

        self.fetch_sql(sql.as_str(), params.as_slice()).await
    }

    async fn add_record(&self, query: InsertQuery) -> Result<()> {
        let (sql, params) = query.build();
        self.execute_sql(sql.as_str(), params.as_slice()).await?;
        Ok(())
    }

    async fn replace(&self, query: ReplaceQuery) -> Result<()> {
        let (sql, params) = query.build();
        self.execute_sql(sql.as_str(), params.as_slice()).await?;
        Ok(())
    }

    async fn update(&self, query: UpdateQuery) -> Result<u64> {
        let (sql, params) = query.build();
        self.execute_sql(sql.as_str(), params.as_slice()).await
    }

    async fn delete(&self, query: DeleteQuery) -> Result<u64> {
        let (sql, params) = query.build();
        self.execute_sql(sql.as_str(), params.as_slice()).await
    }

    async fn increment(&self, query: IncrementQuery) -> Result<()> {
        let (sql, params) = query.build();
        self.execute_sql(sql.as_str(), params.as_slice()).await?;
        Ok(())
    }

    async fn count(&self, query: CountQuery) -> Result<u64> {
        let (sql, params) = query.build();

        let rows = self.fetch_sql(&sql, &params).await?;

        if let Some(value) = rows.first() {
            return value.require_u64("count");
        }

        Ok(0)
    }
}

#[async_trait]
impl DatabaseInterface for SqliteDatabase {
    async fn transact(&self) -> Result<Box<dyn DatabaseInterface>> {
        let conn = self.connect().await?;

        let tx = conn
            .transaction()
            .await
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?;

        Ok(Box::new(SqliteTransaction::new(tx, conn)))
    }

    // Starts a transaction that takes the write lock before the first read.
    //
    // A deferred transaction takes no lock at BEGIN, so a read establishes a
    // WAL snapshot and the first write has to promote it. If another
    // connection committed in between, SQLite fails the promotion with
    // BUSY_SNAPSHOT, and waiting cannot fix a stale snapshot. Taking the lock
    // up front turns contention into a normal wait covered by busy_timeout.
    //
    // Each attempt opens a fresh connection so a failed BEGIN IMMEDIATE cannot
    // leave a half-open transaction behind. Nothing has executed at that
    // point, so a retry cannot duplicate work.
    async fn transact_immediate(&self) -> Result<Box<dyn DatabaseInterface>> {
        if self.is_remote {
            return self.transact().await;
        }

        retry_on_busy(|| async {
            let conn = self.connect().await?;

            let tx = conn
                .transaction_with_behavior(TransactionBehavior::Immediate)
                .await
                .map_err(Error::from)?;

            Ok(Box::new(SqliteTransaction::new(tx, conn)) as Box<dyn DatabaseInterface>)
        })
        .await
    }

    async fn commit(&self) -> Result<()> {
        Err(Error::DatabaseQuery("No transaction started".to_string()))
    }

    async fn rollback(&self) -> Result<()> {
        Err(Error::DatabaseQuery("No transaction started".to_string()))
    }

    async fn fetch_sql(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>> {
        let start = Instant::now();
        let conn = self.connect().await?;

        let mut stmt = conn.prepare(query).await.map_err(|e| {
            Error::DatabaseQuery(format!("Error preparing SQL statement: {e}; query={query}"))
        })?;

        let mut rows = stmt.query(params_from_iter(params)).await.map_err(|e| {
            Error::DatabaseQuery(format!("Error executing SQL statement: {e}; query={query}"))
        })?;

        let mut res: Vec<Attributes> = Vec::new();

        while let Some(row) = rows.next().await? {
            res.push(row.into());
        }

        let duration = start.elapsed();

        debug!(
            "Read {} rows in {}ms for query: {query}",
            res.len(),
            duration.as_millis()
        );

        Ok(res)
    }

    async fn execute_sql(&self, query: &str, params: &[Value]) -> Result<u64> {
        retry_on_busy(|| async {
            let conn = self.connect().await?;

            let rows = conn
                .execute(query, params_from_iter(params.to_vec()))
                .await
                .map_err(Error::from)?;

            Ok(rows)
        })
        .await
    }

    async fn execute_batch(&self, query: &str) -> Result<()> {
        retry_on_busy(|| async {
            let conn = self.connect().await?;

            conn.execute_batch(query).await.map_err(|e| {
                Error::DatabaseQuery(format!("Error executing SQL batch: {e}; query={query}"))
            })?;

            Ok(())
        })
        .await
    }

    async fn get_record(&self, query: SelectQuery) -> Result<Option<Attributes>> {
        let query = query.with_limit(1);
        let records = self.get_records(query).await?;
        Ok(records.first().cloned())
    }

    async fn get_records(&self, query: SelectQuery) -> Result<Vec<Attributes>> {
        let (sql, params) = query.build();

        self.fetch_sql(sql.as_str(), params.as_slice())
            .await
            .map_err(|e| Error::DatabaseQuery(format!("SQL query failed: {e}; SQL={sql}")))
    }

    async fn add_record(&self, query: InsertQuery) -> Result<()> {
        let (sql, params) = query.build();
        self.execute_sql(sql.as_str(), params.as_slice()).await?;
        Ok(())
    }

    async fn replace(&self, query: ReplaceQuery) -> Result<()> {
        let (sql, params) = query.build();
        self.execute_sql(sql.as_str(), params.as_slice()).await?;
        Ok(())
    }

    async fn update(&self, query: UpdateQuery) -> Result<u64> {
        let (sql, params) = query.build();
        self.execute_sql(sql.as_str(), params.as_slice()).await
    }

    async fn delete(&self, query: DeleteQuery) -> Result<u64> {
        let (sql, params) = query.build();
        self.execute_sql(sql.as_str(), params.as_slice()).await
    }

    async fn increment(&self, query: IncrementQuery) -> Result<()> {
        let (sql, params) = query.build();
        self.execute_sql(sql.as_str(), params.as_slice()).await?;
        Ok(())
    }

    async fn count(&self, query: CountQuery) -> Result<u64> {
        let (sql, params) = query.build();

        let rows = self.fetch_sql(&sql, &params).await?;

        if let Some(value) = rows.first() {
            return value.require_u64("count");
        }

        Ok(0)
    }
}

impl Clone for SqliteDatabase {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            is_remote: self.is_remote,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{retry_on_busy, SqliteDatabase, BUSY_ATTEMPTS};
    use crate::infra::database::base::DatabaseInterface;
    use crate::types::{Error, Result};
    use libsql::Builder;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test]
    async fn test_retry_on_busy_succeeds_after_lock_failures() {
        let calls = AtomicU32::new(0);

        let result = retry_on_busy(|| async {
            let attempt = calls.fetch_add(1, Ordering::SeqCst) + 1;

            if attempt < 3 {
                return Err(Error::DatabaseQuery(
                    "SQLite failure: `database is locked`".to_string(),
                ));
            }

            Ok(7)
        })
        .await;

        assert_eq!(result.expect("Error after retries."), 7);
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_on_busy_gives_up_after_attempts() {
        let calls = AtomicU32::new(0);

        let result: Result<u64> = retry_on_busy(|| async {
            calls.fetch_add(1, Ordering::SeqCst);

            Err(Error::DatabaseQuery(
                "SQLite failure: `database is locked`".to_string(),
            ))
        })
        .await;

        assert!(result.is_err());
        assert_eq!(calls.load(Ordering::SeqCst), BUSY_ATTEMPTS);
    }

    #[tokio::test]
    async fn test_retry_on_busy_does_not_retry_other_errors() {
        let calls = AtomicU32::new(0);

        let result: Result<u64> = retry_on_busy(|| async {
            calls.fetch_add(1, Ordering::SeqCst);

            Err(Error::DatabaseQuery(
                "SQLite failure: UNIQUE constraint failed: files.id".to_string(),
            ))
        })
        .await;

        assert!(result.is_err());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_transact_immediate_takes_the_write_lock() {
        let path = SqliteDatabase::get_temporary_path().expect("temporary database path");
        let database = SqliteDatabase::from_local(&path)
            .await
            .expect("local database");

        let transaction = database
            .transact_immediate()
            .await
            .expect("immediate transaction");

        let competitor = Builder::new_local(&path)
            .build()
            .await
            .expect("second handle")
            .connect()
            .expect("second connection");

        // Keep the test fast: the production timeout is 5 s.
        competitor
            .execute_batch("PRAGMA busy_timeout = 50;")
            .await
            .expect("busy timeout pragma");

        let blocked = competitor
            .execute("INSERT INTO roles (id) VALUES ('blocked')", ())
            .await;

        match blocked {
            Ok(_) => panic!("an immediate transaction must block a second writer"),
            Err(e) => assert!(Error::from(e).is_transient_lock()),
        }

        transaction.commit().await.expect("commit");

        competitor
            .execute("INSERT INTO roles (id) VALUES ('after-commit')", ())
            .await
            .expect("write after commit");
    }
}
