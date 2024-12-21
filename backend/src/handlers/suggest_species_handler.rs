use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct SuggestSpeciesHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl SuggestSpeciesHandler {
    pub async fn handle(&self, user_id: u64) -> Result<Vec<String>> {
        self.db.find_recent_species(user_id).await
    }
}

impl Locatable for SuggestSpeciesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
