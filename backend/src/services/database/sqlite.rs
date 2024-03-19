/**
 * Database client implementation for SQLite.
 *
 * Uses async-sqlite.
 *
 * @docs https://docs.rs/async-sqlite/latest/async_sqlite/
 */

use async_sqlite::{Pool, PoolBuilder, JournalMode};
use async_trait::async_trait;
use log::{debug, error, info};

use crate::Result;
use crate::errors::Error;
use crate::objects::{Bounds, TreeInfo, TreeList};
use crate::utils::get_sqlite_path;
use crate::services::database::r#trait::Database;

pub struct SqliteDatabase {
    pub pool: Pool,
}

impl SqliteDatabase {
    pub async fn new() -> Result<Self> {
        let path = get_sqlite_path()?;

        Ok(Self {
            pool: Self::create_pool(&path).await?,
        })
    }

    async fn create_pool(path: &str) -> Result<Pool> {
        match path {
            ":memory:" => Self::create_memory_pool().await,
            _ => Self::create_file_pool(path).await,
        }
    }

    async fn create_file_pool(path: &str) -> Result<Pool> {
        info!("Using SQLite database from {}.", path);

        let pool = match PoolBuilder::new().path(path).journal_mode(JournalMode::Wal).open().await {
            Ok(value) => value,
            Err(e) => {
                error!("Error connecting to the database: {}", e);
                return Err(Error::DatabaseConnect);
            },
        };

        Ok(pool)
    }

    async fn create_memory_pool() -> Result<Pool> {
        info!("Using an in-memory SQLite database.");

        let pool = match PoolBuilder::new().num_conns(1).open().await {
            Ok(value) => value,
            Err(e) => {
                error!("Error connecting to the database: {}", e);
                return Err(Error::DatabaseConnect);
            },
        };

        Self::setup_memory_db(&pool).await?;

        Ok(pool)
    }

    async fn setup_memory_db(pool: &Pool) -> Result<()> {
        let script = include_str!("../../../dev/schema-sqlite.sql");

        let res = pool.conn_mut(move |conn| {
            let tx = conn.transaction()?;
            tx.execute_batch(script)?;
            tx.commit()
        }).await;

        let _ = match res {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error setting up in-memory database: {}", e);
                Err(Error::DatabaseQuery)
            },
        };

        debug!("Memory database initialized.");

        Ok(())
    }

    #[allow(unused)]
    async fn execute(&self, script: String) -> Result<()> {
        let res = self.pool.conn(move |conn| {
            conn.execute(&script, [])
        }).await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error executing SQL script: {}", e);
                Err(Error::DatabaseQuery)
            },
        }
    }
}

#[async_trait]
impl Database for SqliteDatabase {
    /**
     * Read all trees from the database.
     *
     * https://docs.rs/rusqlite/0.30.0/rusqlite/index.html
     */
    async fn get_trees(&self, bounds: Bounds) -> Result<TreeList> {
        let trees = self.pool.conn(move |conn| {
            let mut stmt = conn.prepare("SELECT id, lat, lon FROM trees WHERE lat <= ? AND lat >= ? AND lon <= ? AND lon >= ?")?;
            let mut rows = stmt.query([bounds.n, bounds.s, bounds.e, bounds.w])?;

            let mut trees: Vec<TreeInfo> = Vec::new();

            while let Some(row) = rows.next()? {
                let id: u64 = row.get(0)?;
                let lat: f64 = row.get(1)?;
                let lon: f64 = row.get(2)?;

                trees.push(TreeInfo {
                    id,
                    lat,
                    lon,
                });
            }

            Ok(trees)
        }).await?;

        Ok(TreeList {
            trees,
        })
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
    use std::env;
    use env_logger;

    #[tokio::test]
    async fn test_get_trees() {
        env::set_var("RUST_LOG", "debug");
        env::set_var("TREEMAP_SQLITE_PATH", ":memory:");

        env_logger::init();

        let db = SqliteDatabase::new().await.unwrap();
        db.execute(include_str!("./fixtures/trees.sql").to_string()).await.unwrap();

        let bounds = Bounds {
            n: 90.0,
            e: 180.0,
            s: -90.0,
            w: -180.0,
        };

        let trees = match db.get_trees(bounds).await {
            Ok(value) => value,

            Err(e) => {
                panic!("Error reading trees from the database: {}", e);
            },
        };

        assert_eq!(trees.trees.len(), 1);
    }
}
