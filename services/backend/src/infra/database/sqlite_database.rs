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
use libsql::{Builder, Connection, Database, Transaction};
use log::{debug, info};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{Mutex, MutexGuard};

// This is where temporary database files are created to simulate
// the in-memory database, which doesn't really work as we need with libsql.
const TEMP_DB_DIR: &str = "var/test-files";

pub struct SqliteDatabase {
    pool: Arc<Database>,
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
        let tx = tx_lock.as_mut().expect("Transaction is open");

        let rows = tx
            .execute(query, params_from_iter(params.to_vec()))
            .await
            .map_err(|e| {
                Error::DatabaseQuery(format!("Error executing SQL statement: {e}; query={query}"))
            })?;

        Ok(rows)
    }

    async fn fetch_sql(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>> {
        let start = Instant::now();
        let mut tx_lock = self.lock().await?;
        let tx = tx_lock.as_mut().expect("Transaction is open");

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

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA busy_timeout = 5000;",
        )
        .await
        .map_err(|e| Error::DatabaseQuery(format!("Error setting database pragmas: {e}")))?;

        if cfg!(test) {
            let schema = include_str!("../../../dev/schema-sqlite.sql");
            conn.execute_batch(schema).await?;
        }

        Ok(())
    }

    async fn connect(&self) -> Result<Connection> {
        let conn = self
            .pool
            .connect()
            .map_err(|e| Error::DatabaseConnect(e.to_string()))?;

        conn.execute_batch("PRAGMA busy_timeout = 5000;")
            .await
            .map_err(|e| Error::DatabaseQuery(format!("Error setting busy_timeout: {e}")))?;

        Ok(conn)
    }
}

#[async_trait]
impl DatabaseInterface for SqliteTransaction {
    async fn transact(&self) -> Result<Box<dyn DatabaseInterface>> {
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
        let tx = tx_lock.as_mut().expect("Transaction is open");

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
        let conn = self.connect().await?;

        let rows = conn
            .execute(query, params_from_iter(params.to_vec()))
            .await
            .map_err(|e| {
                Error::DatabaseQuery(format!("Error executing SQL statement: {e}; query={query}"))
            })?;

        Ok(rows)
    }

    async fn execute_batch(&self, query: &str) -> Result<()> {
        let conn = self.connect().await?;

        conn.execute_batch(query).await.map_err(|e| {
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
        }
    }
}
