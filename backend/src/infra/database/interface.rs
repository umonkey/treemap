//! Database client factory.
//!
//! This code does not implement any database, but it creates the right
//! driver according to the configuration.

use super::attributes::Attributes;
use super::base::DatabaseInterface;
use super::sqlite_database::SqliteDatabase;
use super::value::Value;
use crate::common::database::queries::{
    CountQuery, DeleteQuery, IncrementQuery, InsertQuery, ReplaceQuery, SelectQuery, UpdateQuery,
};
use crate::config::Secrets;
use crate::infra::config::Config;
use crate::services::*;
use crate::types::{Error, Result, SpeciesRecord, TreeRecord};
use log::{debug, error};
use std::sync::Arc;

pub struct Database {
    db: Arc<dyn DatabaseInterface>,
}

impl Database {
    pub async fn get_record(&self, query: SelectQuery) -> Result<Option<Attributes>> {
        self.db.get_record(query).await
    }

    pub async fn get_records(&self, query: SelectQuery) -> Result<Vec<Attributes>> {
        self.db.get_records(query).await
    }

    pub async fn add_record(&self, query: InsertQuery) -> Result<()> {
        self.db.add_record(query).await
    }

    pub async fn replace(&self, query: ReplaceQuery) -> Result<()> {
        self.db.replace(query).await
    }

    pub async fn update(&self, query: UpdateQuery) -> Result<()> {
        self.db.update(query).await
    }

    pub async fn delete(&self, query: DeleteQuery) -> Result<()> {
        self.db.delete(query).await
    }

    pub async fn increment(&self, query: IncrementQuery) -> Result<()> {
        self.db.increment(query).await
    }

    pub async fn count(&self, query: CountQuery) -> Result<u64> {
        self.db.count(query).await
    }

    pub async fn sql(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>> {
        self.db.sql(query, params).await
    }

    pub async fn find_species(&self, query: &str) -> Result<Vec<SpeciesRecord>> {
        self.db.find_species(query).await
    }

    pub async fn find_streets(&self, query: &str) -> Result<Vec<String>> {
        self.db.find_streets(query).await
    }

    pub async fn find_recent_species(&self, user_id: u64) -> Result<Vec<String>> {
        self.db.find_recent_species(user_id).await
    }

    pub async fn get_species_stats(&self) -> Result<Vec<(String, u64)>> {
        self.db.get_species_stats().await
    }

    pub async fn get_top_streets(&self, count: u64) -> Result<Vec<(String, u64)>> {
        self.db.get_top_streets(count).await
    }

    pub async fn get_state_stats(&self) -> Result<Vec<(String, u64)>> {
        self.db.get_state_stats().await
    }

    pub async fn get_species_mismatch(&self, count: u64, skip: u64) -> Result<Vec<TreeRecord>> {
        self.db.get_species_mismatch(count, skip).await
    }

    pub async fn get_heatmap(&self, after: u64, before: u64) -> Result<Vec<(String, u64)>> {
        self.db.get_heatmap(after, before).await
    }

    pub async fn get_user_heatmap(
        &self,
        after: u64,
        before: u64,
        user_id: u64,
    ) -> Result<Vec<(String, u64)>> {
        self.db.get_user_heatmap(after, before, user_id).await
    }
}

impl Locatable for Database {
    fn create(locator: &Locator) -> Result<Self> {
        let config = locator.get::<Config>()?;

        if config.database == "turso" {
            let secrets = locator.get::<Secrets>()?;

            debug!("Setting up a Turso database.");

            let url: String = config
                .turso_url
                .clone()
                .ok_or(Error::Config("turso_url not set".to_string()))?;

            let token: String = secrets
                .turso_token
                .clone()
                .ok_or(Error::Config("turso_token not set".to_string()))?;

            let db = futures::executor::block_on(SqliteDatabase::from_remote(&url, &token))?;

            Ok(Self { db: Arc::new(db) })
        } else if config.database == "memory" {
            let db = futures::executor::block_on(SqliteDatabase::from_memory())?;

            Ok(Self { db: Arc::new(db) })
        } else if config.database == "sqlite" {
            let path = &config.sqlite_path;
            let db = futures::executor::block_on(SqliteDatabase::from_local(path))?;

            Ok(Self { db: Arc::new(db) })
        } else {
            error!("Unknown database type: {}", config.database);
            Err(Error::Config("unknown database type".to_string()))
        }
    }
}
