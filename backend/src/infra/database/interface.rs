//! Database client factory.
//!
//! This code does not implement any database, but it creates the right
//! driver according to the configuration.

use super::attributes::Attributes;
use super::base::DatabaseInterface;
use super::queries::*;
use super::sqlite_database::SqliteDatabase;
use super::value::Value;
use crate::infra::config::Config;
use crate::infra::secrets::Secrets;
use crate::types::{Error, Result};
use log::{debug, error};
use std::sync::Arc;

pub struct Database {
    db: Arc<dyn DatabaseInterface>,
}

impl Database {
    #[allow(dead_code)]
    pub async fn transact(&self) -> Result<Self> {
        let db = self.db.transact().await?;
        Ok(Self { db: Arc::from(db) })
    }

    #[allow(dead_code)]
    pub async fn commit(&self) -> Result<()> {
        self.db.commit().await
    }

    #[allow(dead_code)]
    pub async fn rollback(&self) -> Result<()> {
        self.db.rollback().await
    }

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

    pub async fn update(&self, query: UpdateQuery) -> Result<u64> {
        self.db.update(query).await
    }

    pub async fn delete(&self, query: DeleteQuery) -> Result<u64> {
        self.db.delete(query).await
    }

    pub async fn increment(&self, query: IncrementQuery) -> Result<()> {
        self.db.increment(query).await
    }

    pub async fn count(&self, query: CountQuery) -> Result<u64> {
        self.db.count(query).await
    }

    pub async fn fetch_sql(&self, query: &str, params: &[Value]) -> Result<Vec<Attributes>> {
        self.db.fetch_sql(query, params).await
    }

    pub async fn execute_sql(&self, query: &str, params: &[Value]) -> Result<()> {
        self.db.execute_sql(query, params).await
    }

    #[allow(dead_code)]
    pub async fn execute(&self, query: &str) -> Result<()> {
        self.db.execute(query).await
    }
}

impl Database {
    pub async fn new(config: &Config, secrets: &Secrets) -> Result<Self> {
        if config.database == "turso" {
            debug!("Setting up a Turso database.");

            let url: String = config
                .turso_url
                .clone()
                .ok_or(Error::Config("turso_url not set".to_string()))?;

            let token: String = secrets
                .turso_token
                .clone()
                .ok_or(Error::Config("turso_token not set".to_string()))?;

            let db = SqliteDatabase::from_remote(&url, &token).await?;

            Ok(Self { db: Arc::new(db) })
        } else if config.database == "memory" {
            let db = SqliteDatabase::from_memory().await?;

            Ok(Self { db: Arc::new(db) })
        } else if config.database == "sqlite" {
            let path = &config.sqlite_path;
            let db = SqliteDatabase::from_local(path).await?;

            Ok(Self { db: Arc::new(db) })
        } else {
            error!("Unknown database type: {}", config.database);
            Err(Error::Config("unknown database type".to_string()))
        }
    }
}
