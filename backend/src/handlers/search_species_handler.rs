use crate::infra::database::Database;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct SearchSpeciesHandler {
    db: Arc<Database>,
}

impl SearchSpeciesHandler {
    pub async fn handle(&self, query: &str) -> Result<Vec<SpeciesRecord>> {
        self.db.find_species(query).await
    }
}

impl Locatable for SearchSpeciesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<Database>()?;
        Ok(Self { db })
    }
}
