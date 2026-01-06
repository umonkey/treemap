//! Database client factory.
//!
//! This code does not implement any database, but it creates the right
//! driver according to the configuration.

use super::database_interface::*;
use super::turso_database::*;
use crate::config::{Config, Secrets};
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct PreferredDatabase {
    pub db: Arc<dyn DatabaseInterface>,
}

impl PreferredDatabase {
    pub fn driver(&self) -> Arc<dyn DatabaseInterface> {
        self.db.clone()
    }
}

impl Locatable for PreferredDatabase {
    fn create(locator: &Locator) -> Result<Self> {
        let config = locator.get::<Config>()?;

        if config.database == "turso" {
            let secrets = locator.get::<Secrets>()?;

            let url: String = config.turso_url.clone().ok_or(Error::Config)?;
            let token: String = secrets.require("TURSO_TOKEN")?;

            let db = futures::executor::block_on(TursoDatabase::from_remote(&url, &token))?;

            Ok(Self { db: Arc::new(db) })
        } else {
            let path = &config.sqlite_path;
            let db = futures::executor::block_on(TursoDatabase::from_local(path))?;

            Ok(Self { db: Arc::new(db) })
        }
    }
}
