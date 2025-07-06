use super::database_interface::*;
use super::sqlite_database::*;
use crate::config::Config;
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
        let db = futures::executor::block_on(SqliteDatabase::new(config))?;
        Ok(Self { db: Arc::new(db) })
    }
}
