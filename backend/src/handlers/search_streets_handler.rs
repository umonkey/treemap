//! Returns street addresses that contain a substring.

use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct SearchStreetsHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl SearchStreetsHandler {
    pub async fn handle(&self, query: &str) -> Result<Vec<PublicStreetInfo>> {
        let records = self.db.find_streets(query).await?;
        let species = records.iter().map(PublicStreetInfo::from_address).collect();
        Ok(species)
    }
}

impl Locatable for SearchStreetsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
