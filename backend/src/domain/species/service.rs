use super::models::Species;
use super::report::format_species_report;
use super::schemas::SpeciesStats;
use crate::infra::database::Database;
use crate::services::{Locatable, Locator};
use crate::types::Result;
use std::sync::Arc;

pub struct SpeciesService {
    db: Arc<Database>,
}

impl SpeciesService {
    pub async fn search(&self, query: &str) -> Result<Vec<Species>> {
        self.db.find_species(query).await
    }

    pub async fn suggest(&self, user_id: u64) -> Result<Vec<String>> {
        self.db.find_recent_species(user_id).await
    }

    pub async fn get_stats(&self) -> Result<Vec<SpeciesStats>> {
        let items = self.db.get_species_stats().await?;
        let report = format_species_report(items);
        Ok(report)
    }
}

impl Locatable for SpeciesService {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<Database>()?;
        Ok(Self { db })
    }
}
