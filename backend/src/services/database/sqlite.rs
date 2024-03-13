/**
 * Database client implementation for SQLite.
 *
 * Uses async-sqlite.
 *
 * @docs https://docs.rs/async-sqlite/latest/async_sqlite/
 */

use async_sqlite::{Pool, PoolBuilder, JournalMode};
use log::error;

use crate::Result;
use crate::errors::Error;
use crate::objects::{TreeInfo, TreeList};
use crate::utils::getenv_string;

pub struct SqliteDatabase {
    pub pool: Pool,
}

impl SqliteDatabase {
    pub async fn init() -> Result<Self> {
        let path = getenv_string("TREEMAP_SQLITE_PATH")?;

        let pool = match PoolBuilder::new().path(path).journal_mode(JournalMode::Wal).open().await {
            Ok(value) => value,
            Err(e) => {
                error!("Error connecting to the database: {}", e);
                return Err(Error::DatabaseConnect);
            },
        };

        Ok(Self {
            pool,
        })
    }

    /**
     * Read all trees from the database.
     *
     * https://docs.rs/rusqlite/0.30.0/rusqlite/index.html
     */
    #[allow(dead_code)]
    pub async fn get_trees(&self) -> Result<TreeList> {
        let res = self.pool.conn(|conn| {
            let mut stmt = conn.prepare("SELECT id, lat, lon FROM trees")?;
            let mut rows = stmt.query([])?;

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
        }).await;

        let trees = match res {
            Ok(value) => value,

            Err(e) => {
                error!("Error reading trees from the database: {}", e);
                return Err(Error::DatabaseQuery);
            },
        };

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
