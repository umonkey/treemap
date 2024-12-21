use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct SearchSpeciesHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl SearchSpeciesHandler {
    pub async fn handle(&self, query: &str) -> Result<Vec<PublicSpeciesInfo>> {
        let records = self.db.find_species(query).await?;
        let species = records.iter().map(PublicSpeciesInfo::from_record).collect();
        Ok(species)
    }
}

impl Locatable for SearchSpeciesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
