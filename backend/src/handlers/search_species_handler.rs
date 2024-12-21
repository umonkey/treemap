use crate::services::Database;
use crate::services::SqliteDatabase;
use crate::services::{Locatable, Locator};
use crate::types::*;
use std::sync::Arc;

pub struct SearchSpeciesHandler {
    db: Arc<dyn Database + Send + Sync>,
}

impl SearchSpeciesHandler {
    pub fn new(db: Arc<SqliteDatabase>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, query: &str) -> Result<Vec<PublicSpeciesInfo>> {
        let records = self.db.find_species(query).await?;
        let species = records.iter().map(PublicSpeciesInfo::from_record).collect();
        Ok(species)
    }
}

impl Locatable for SearchSpeciesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<SqliteDatabase>()?;
        Ok(Self::new(db))
    }
}
