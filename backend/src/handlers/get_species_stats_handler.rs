use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetSpeciesStatsHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl GetSpeciesStatsHandler {
    pub async fn handle(&self) -> Result<Vec<SpeciesStatsResponse>> {
        let res = self.db.get_species_stats().await?;

        let res = res
            .iter()
            .map(|(species, count)| SpeciesStatsResponse {
                name: species.clone(),
                count: *count,
                subspecies: vec![],
            })
            .collect();

        Ok(res)
    }
}

impl Locatable for GetSpeciesStatsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
