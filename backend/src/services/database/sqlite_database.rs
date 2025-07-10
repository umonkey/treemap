//! Database client implementation for SQLite.
//!
//! Uses async-sqlite.
//!
//! Probably nee to split the core database code into a separate class,
//! and the queue implementation should also be separate, with its own database.
//!
//! @docs https://docs.rs/async-sqlite/latest/async_sqlite/

use crate::common::database::queries::*;
use crate::config::Config;
use crate::services::*;
use crate::types::*;
use crate::utils::get_timestamp;
use async_sqlite::{JournalMode, Pool, PoolBuilder};
use async_trait::async_trait;
use log::{debug, error, info};
use rusqlite::params_from_iter;
use rusqlite::types::Value;
use std::cmp::Ordering;
use std::sync::Arc;

const SUGGEST_WINDOW: u64 = 3600 * 24; // 24 hours

pub struct SqliteDatabase {
    pub pool: Pool,
}

impl SqliteDatabase {
    pub async fn new(config: Arc<Config>) -> Result<Self> {
        let path = config.sqlite_path.clone();

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
        info!("Using SQLite database from {path}.");

        let pool = match PoolBuilder::new()
            .path(path)
            .journal_mode(JournalMode::Wal)
            .open()
            .await
        {
            Ok(value) => value,
            Err(e) => {
                error!("Error connecting to the database: {e:?}");
                return Err(Error::DatabaseConnect);
            }
        };

        Ok(pool)
    }

    async fn create_memory_pool() -> Result<Pool> {
        info!("Using an in-memory SQLite database.");

        let pool = match PoolBuilder::new().num_conns(1).open().await {
            Ok(value) => value,
            Err(e) => {
                error!("Error connecting to the database: {e}");
                return Err(Error::DatabaseConnect);
            }
        };

        Self::setup_memory_db(&pool).await?;

        Ok(pool)
    }

    async fn setup_memory_db(pool: &Pool) -> Result<()> {
        let script = include_str!("../../../dev/schema-sqlite.sql");
        Self::execute_pool(pool, script.to_string()).await?;

        debug!("Memory database initialized.");

        Ok(())
    }

    #[allow(unused)]
    pub async fn execute(&self, script: String) -> Result<()> {
        Self::execute_pool(&self.pool, script).await
    }

    async fn execute_pool(pool: &Pool, script: String) -> Result<()> {
        let res = pool.conn(move |conn| conn.execute_batch(&script)).await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error executing SQL script: {e}");
                Err(Error::DatabaseQuery)
            }
        }
    }

    /**
     * Custom case-insensitive collation for SQLite.
     *
     * Unlike the LOWER() function, this supports Unicode.
     */
    fn case_insensitive_collation(a: &str, b: &str) -> Ordering {
        a.to_lowercase().cmp(&b.to_lowercase())
    }

    fn pack_record(row: &rusqlite::Row, fields: &Vec<&str>) -> rusqlite::Result<Attributes> {
        let mut props = Vec::<(String, Value)>::new();

        for field in fields {
            let value = row.get(*field)?;
            props.push((field.to_string(), value));
        }

        Ok(Attributes::from(&props))
    }
}

impl Locatable for SqliteDatabase {
    fn create(locator: &Locator) -> Result<Self> {
        let config = locator.get::<Config>()?;
        futures::executor::block_on(Self::new(config))
    }
}

#[async_trait]
impl DatabaseInterface for SqliteDatabase {
    async fn get_record(&self, query: SelectQuery) -> Result<Option<Attributes>> {
        let query = query.with_limit(1);
        let records = self.get_records(query).await?;
        Ok(records.first().cloned())
    }

    async fn sql(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>> {
        let query = query.to_string();
        let params = params.to_vec();

        let record = self
            .pool
            .conn(move |conn| {
                let mut stmt = match conn.prepare(query.as_str()) {
                    Ok(value) => value,

                    Err(e) => {
                        error!("Error preparing SQL statement: {e}");
                        return Err(e);
                    }
                };

                let mut rows = stmt.query(params_from_iter(params.iter())).map_err(|e| {
                    error!("Error executing SQL statement: {e}");
                    e
                })?;

                let mut records = Vec::new();

                while let Some(row) = rows.next()? {
                    let fields = row.as_ref().column_names();
                    let rec = Self::pack_record(row, &fields)?;
                    records.push(rec);
                }

                Ok(records)
            })
            .await?;

        Ok(record)
    }

    async fn get_records(&self, query: SelectQuery) -> Result<Vec<Attributes>> {
        let (sql, params) = query.build();
        self.sql(sql.as_str(), params.as_slice()).await
    }

    async fn add_record(&self, query: InsertQuery) -> Result<()> {
        self.pool
            .conn(move |conn| {
                let (sql, params) = query.build();

                match conn.execute(sql.as_str(), params_from_iter(params)) {
                    Ok(_) => (),

                    Err(e) => {
                        error!("Error adding a record to the database: {e}");
                        return Err(e);
                    }
                };

                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn replace(&self, query: ReplaceQuery) -> Result<()> {
        self.pool
            .conn(move |conn| {
                let (sql, params) = query.build();

                match conn.execute(sql.as_str(), params_from_iter(params)) {
                    Ok(_) => (),

                    Err(e) => {
                        error!("Error replacing a record to the database: {e}");
                        return Err(e);
                    }
                };

                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn update(&self, query: UpdateQuery) -> Result<()> {
        self.pool
            .conn(move |conn| {
                let (sql, params) = query.build();

                match conn.execute(sql.as_str(), params_from_iter(params)) {
                    Ok(_) => (),

                    Err(e) => {
                        error!("Error updating database: {e}");
                        return Err(e);
                    }
                };

                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn delete(&self, query: DeleteQuery) -> Result<()> {
        self.pool
            .conn(move |conn| {
                let (sql, params) = query.build();

                match conn.execute(sql.as_str(), params_from_iter(params)) {
                    Ok(_) => (),

                    Err(e) => {
                        error!("Error deleting record: {e}");
                        return Err(e);
                    }
                };

                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn increment(&self, query: IncrementQuery) -> Result<()> {
        self.pool
            .conn(move |conn| {
                let (sql, params) = query.build();

                match conn.execute(sql.as_str(), params_from_iter(params)) {
                    Ok(_) => (),

                    Err(e) => {
                        error!("Error incrementing a value: {e}");
                        return Err(e);
                    }
                };

                Ok(())
            })
            .await?;

        Ok(())
    }

    /**
     * Count all trees that still exist.
     */
    async fn count(&self, query: CountQuery) -> Result<u64> {
        let count = self
            .pool
            .conn(move |conn| {
                let (sql, params) = query.build();

                let mut stmt = match conn.prepare(sql.as_str()) {
                    Ok(value) => value,

                    Err(e) => {
                        error!("Error preparing SQL statement: {e}");
                        return Err(e);
                    }
                };

                let mut rows = stmt.query(params_from_iter(params.iter())).map_err(|e| {
                    error!("Error executing SQL statement: {e}");
                    e
                })?;

                match rows.next()? {
                    Some(row) => Ok(row.get(0)?),
                    None => Ok(0),
                }
            })
            .await?;

        Ok(count)
    }

    async fn find_species(&self, query: &str) -> Result<Vec<SpeciesRecord>> {
        let pattern = format!("%{}%", query.trim().to_lowercase());

        let species = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT name, local, keywords, wikidata_id FROM species WHERE name LIKE ?1 OR local LIKE ?1 OR keywords LIKE ?1 ORDER BY name LIMIT 10") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {e}");
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([pattern]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {e}");
                    return Err(e);
                },
            };

            let mut species: Vec<SpeciesRecord> = Vec::new();

            while let Some(row) = rows.next()? {
                species.push(SpeciesRecord {
                    name: row.get(0)?,
                    local: row.get(1)?,
                    keywords: row.get(2)?,
                    wikidata_id: row.get(3)?,
                });
            }

            Ok(species)
        }).await?;

        Ok(species)
    }

    async fn find_streets(&self, query: &str) -> Result<Vec<String>> {
        let pattern = format!("%{}%", query.trim().to_lowercase());

        let streets = self
            .pool
            .conn(move |conn| {
                let mut stmt = match conn.prepare(
                    "SELECT DISTINCT address FROM trees WHERE state <> 'gone' AND address LIKE ?1 ORDER BY address LIMIT 10",
                ) {
                    Ok(value) => value,

                    Err(e) => {
                        error!("Error preparing SQL statement: {e}");
                        return Err(e);
                    }
                };

                let mut rows = match stmt.query([pattern]) {
                    Ok(value) => value,

                    Err(e) => {
                        error!("Error executing SQL statement: {e}");
                        return Err(e);
                    }
                };

                let mut streets: Vec<String> = Vec::new();

                while let Some(row) = rows.next()? {
                    streets.push(row.get(0)?);
                }

                Ok(streets)
            })
            .await?;

        Ok(streets)
    }

    /**
     * Find most recent species added by a specific user.
     */
    async fn find_recent_species(&self, user_id: u64) -> Result<Vec<String>> {
        let since = get_timestamp() - SUGGEST_WINDOW;

        let items = self.pool.conn(move |conn| {
            conn.create_collation("case_insensitive", Self::case_insensitive_collation)?;

            let mut stmt = match conn.prepare("SELECT species, COUNT(1) AS use_count FROM trees WHERE added_by = ? AND added_at >= ? AND LOWER(species) <> 'unknown' GROUP BY species COLLATE case_insensitive ORDER BY use_count DESC LIMIT 10") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {e}");
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([user_id, since]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {e}");
                    return Err(e);
                },
            };

            let mut values: Vec<String> = Vec::new();

            while let Some(row) = rows.next()? {
                values.push(row.get(0)?);
            }

            Ok(values)
        }).await?;

        Ok(items)
    }

    async fn get_species_stats(&self) -> Result<Vec<(String, u64)>> {
        let res = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT species, COUNT(1) AS cnt FROM trees WHERE state <> 'gone' GROUP BY species ORDER BY cnt DESC, LOWER(species)") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {e}");
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {e}");
                    return Err(e);
                },
            };

            let mut res = Vec::new();

            while let Some(row) = rows.next()? {
                let species: String = row.get(0)?;
                let count: u64 = row.get(1)?;
                res.push((species, count));
            }

            Ok(res)
        }).await?;

        Ok(res)
    }

    async fn get_species_mismatch(&self, count: u64, skip: u64) -> Result<Vec<TreeRecord>> {
        let res = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, osm_id, lat, lon, species, notes, height, circumference, diameter, state, added_at, updated_at, added_by, thumbnail_id, year, address FROM trees WHERE state <> 'gone' AND species <> 'Unknown species' AND species <> 'Unknown' AND species NOT IN (SELECT name FROM species) LIMIT ? OFFSET ?") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {e}");
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([count, skip]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {e}");
                    return Err(e);
                },
            };

            let mut records = Vec::new();

            while let Some(row) = rows.next()? {
                let fields = row.as_ref().column_names();
                let rec = Self::pack_record(row, &fields)?;

                if let Ok(packed) = TreeRecord::from_attributes(&rec) {
                    records.push(packed);
                }
            }

            Ok(records)
        }).await?;

        Ok(res)
    }

    async fn get_top_streets(&self, count: u64) -> Result<Vec<(String, u64)>> {
        let res = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT address, COUNT(1) AS cnt FROM trees WHERE state <> 'gone' AND address IS NOT NULL GROUP BY LOWER(address) ORDER BY cnt DESC, LOWER(address) LIMIT ?") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {e}");
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([count]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {e}");
                    return Err(e);
                },
            };

            let mut res = Vec::new();

            while let Some(row) = rows.next()? {
                let address: String = row.get(0)?;
                let count: u64 = row.get(1)?;
                res.push((address, count));
            }

            Ok(res)
        }).await?;

        Ok(res)
    }

    async fn get_state_stats(&self) -> Result<Vec<(String, u64)>> {
        let res = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT state, COUNT(1) AS cnt FROM trees WHERE state IS NOT NULL GROUP BY state ORDER BY cnt DESC") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {e}");
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {e}");
                    return Err(e);
                },
            };

            let mut res = Vec::new();

            while let Some(row) = rows.next()? {
                let state: String = row.get(0)?;
                let count: u64 = row.get(1)?;
                res.push((state, count));
            }

            Ok(res)
        }).await?;

        Ok(res)
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

    async fn setup() -> SqliteDatabase {
        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        let config = Config::from_default_file().expect("Error reading config");

        let db = SqliteDatabase::new(Arc::new(config))
            .await
            .expect("Error creating database.");

        db.execute(include_str!("./fixtures/init.sql").to_string())
            .await
            .expect("Error installing fixtures.");

        db
    }

    #[tokio::test]
    async fn test_find_species() {
        let db = setup().await;

        db.execute(include_str!("./fixtures/species.sql").to_string())
            .await
            .expect("Error adding species.");

        let species = db
            .find_species(" oak ")
            .await
            .expect("Error finding species.");

        assert_eq!(species.len(), 1, "Could not find species for oak.");
        assert_eq!("Quercus robur", species[0].name);
    }
}
