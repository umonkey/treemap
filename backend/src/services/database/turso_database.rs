//! Database client implementation for Turso.
//!
//! @docs https://docs.turso.tech/sdk/rust/quickstart

use crate::common::database::queries::*;
use crate::infra::database::{Attributes, Value};
use crate::services::*;
use crate::types::*;
use crate::utils::get_timestamp;
use crate::utils::get_unique_id;
use async_trait::async_trait;
use libsql::params_from_iter;
use libsql::{Builder, Database};
use log::{error, info};
use std::sync::Arc;

const SUGGEST_WINDOW: u64 = 3600 * 24; // 24 hours

pub struct TursoDatabase {
    pool: Arc<Database>,
}

impl TursoDatabase {
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

        Self::setup_pool(&pool).await?;

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

    #[allow(unused)]
    pub async fn execute(&self, script: String) -> Result<()> {
        Self::execute_pool(&self.pool, script).await
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

    async fn fetch(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>> {
        let conn = self.pool.connect()?;

        let mut stmt = conn.prepare(query).await.inspect_err(|e| {
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

        Ok(res)
    }

    fn get_temporary_path() -> String {
        let id = get_unique_id().unwrap_or(0);
        format!("var/test-{id}.db")
    }

    async fn setup_pool(pool: &Database) -> Result<()> {
        let conn = pool.connect()?;

        conn.execute_batch("PRAGMA busy_timeout = 5000;").await.inspect_err(|e| {
            error!("Error setting busy_timeout: {e}");
        })?;

        conn.execute_batch("PRAGMA journal_mode = WAL;").await.inspect_err(|e| {
            error!("Error setting journal_mode: {e}");
        })?;

        conn.execute_batch("PRAGMA synchronous = NORMAL;").await.inspect_err(|e| {
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
impl DatabaseInterface for TursoDatabase {
    async fn sql(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>> {
        self.fetch(query, params).await
    }

    async fn get_record(&self, query: SelectQuery) -> Result<Option<Attributes>> {
        let query = query.with_limit(1);
        let records = self.get_records(query).await?;
        Ok(records.first().cloned())
    }

    async fn get_records(&self, query: SelectQuery) -> Result<Vec<Attributes>> {
        let (sql, params) = query.build();
        self.sql(sql.as_str(), params.as_slice()).await
    }

    async fn add_record(&self, query: InsertQuery) -> Result<()> {
        let conn = self.pool.connect()?;

        let (sql, params) = query.build();

        conn.execute(sql.as_str(), params_from_iter(params))
            .await
            .inspect_err(|e| {
                error!("Error adding a record to the database: {e}");
            })?;

        Ok(())
    }

    async fn replace(&self, query: ReplaceQuery) -> Result<()> {
        let conn = self.pool.connect()?;

        let (sql, params) = query.build();

        conn.execute(sql.as_str(), params_from_iter(params))
            .await
            .inspect_err(|e| {
                error!("Error replacing a record to the database: {e}");
            })?;

        Ok(())
    }

    async fn update(&self, query: UpdateQuery) -> Result<()> {
        let conn = self.pool.connect()?;

        let (sql, params) = query.build();

        conn.execute(sql.as_str(), params_from_iter(params))
            .await
            .inspect_err(|e| {
                error!("Error updating database: {e}");
            })?;

        Ok(())
    }

    async fn delete(&self, query: DeleteQuery) -> Result<()> {
        let conn = self.pool.connect()?;

        let (sql, params) = query.build();

        conn.execute(sql.as_str(), params_from_iter(params))
            .await
            .inspect_err(|e| {
                error!("Error deleting record: {e}");
            })?;

        Ok(())
    }

    async fn increment(&self, query: IncrementQuery) -> Result<()> {
        let conn = self.pool.connect()?;

        let (sql, params) = query.build();

        conn.execute(sql.as_str(), params_from_iter(params))
            .await
            .inspect_err(|e| {
                error!("Error incrementing a value: {e}");
            })?;

        Ok(())
    }

    /**
     * Count all trees that still exist.
     */
    async fn count(&self, query: CountQuery) -> Result<u64> {
        let (sql, params) = query.build();

        let rows = self.fetch(&sql, &params).await?;

        if let Some(value) = rows.first() {
            return value.require_u64("count");
        }

        Ok(0)
    }

    async fn find_species(&self, query: &str) -> Result<Vec<SpeciesRecord>> {
        let pattern = format!("%{}%", query.trim().to_lowercase());

        let rows = self.fetch(
            "SELECT name, local, keywords, wikidata_id FROM species WHERE name LIKE ?1 OR local LIKE ?1 OR keywords LIKE ?1 ORDER BY name LIMIT 10",
            &[Value::from(pattern)],
        ).await?;

        let mut species: Vec<SpeciesRecord> = Vec::new();

        for row in rows {
            species.push(SpeciesRecord {
                name: row.require_string("name")?,
                local: row.require_string("local")?,
                keywords: row.require_string("keywords")?,
                wikidata_id: row.require_string("wikidata_id")?,
            });
        }

        Ok(species)
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

    /**
     * Find most recent species added by a specific user.
     */
    async fn find_recent_species(&self, user_id: u64) -> Result<Vec<String>> {
        let since = get_timestamp() - SUGGEST_WINDOW;

        let rows = self.fetch(
            "SELECT species, COUNT(1) AS use_count FROM trees WHERE added_by = ? AND added_at >= ? AND LOWER(species) <> 'unknown' GROUP BY species ORDER BY use_count DESC LIMIT 10",
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

    async fn get_species_mismatch(&self, count: u64, skip: u64) -> Result<Vec<TreeRecord>> {
        let rows = self.fetch(
            "SELECT id, osm_id, lat, lon, species, notes, height, circumference, diameter, state, added_at, updated_at, added_by, thumbnail_id, year, address FROM trees WHERE state <> 'gone' AND species <> 'Unknown species' AND species <> 'Unknown' AND species NOT IN (SELECT name FROM species) LIMIT ? OFFSET ?",
            &[Value::from(count), Value::from(skip)],
        ).await?;

        let mut records = Vec::new();

        for row in rows {
            if let Ok(packed) = TreeRecord::from_attributes(&row) {
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

impl Clone for TursoDatabase {
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

    async fn setup() -> TursoDatabase {
        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        let db = TursoDatabase::from_memory()
            .await
            .expect("Error creating database.");

        db.execute(include_str!("./fixtures/init.sql").to_string())
            .await
            .expect("Error installing fixtures.");

        info!("SQLite database initialized.");

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

    #[tokio::test]
    async fn test_species_stats() {
        let db = setup().await;

        db.execute(include_str!("./fixtures/test_species_stats.sql").to_string())
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

        db.execute(include_str!("./fixtures/test_heatmap.sql").to_string())
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

        db.execute(include_str!("./fixtures/test_heatmap.sql").to_string())
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
