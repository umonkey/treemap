//! Provides a report on species distribution.
//!
//! This reads all non-deleted species from the database, groups them by genus / species,
//! and provides total counts for both.
//!
//! Gone trees and stumps are excluded from the count.

use crate::common::reports::format_species_report;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetSpeciesStatsHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl GetSpeciesStatsHandler {
    pub async fn handle(&self) -> Result<Vec<SpeciesStatsResponse>> {
        let items = self.db.get_species_stats().await?;
        let report = format_species_report(items);
        Ok(report)
    }
}

impl Locatable for GetSpeciesStatsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
