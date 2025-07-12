//! This handler provides a street report produced by multiple components.

use crate::common::database::repositories::*;
use crate::reports::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetStreetReportHandler {
    trees: Arc<TreeRepository>,
    area: Arc<TreesAreaReporter>,
    by_state: Arc<TreesByStateReporter>,
    by_height: Arc<TreesByHeightReporter>,
    by_crown: Arc<TreesByCrownReporter>,
    by_girth: Arc<TreesByGirthReporter>,
    by_species: Arc<TreesBySpeciesReporter>,
}

impl GetStreetReportHandler {
    pub async fn handle(&self, street: &str) -> Result<StreetReport> {
        let trees = self.find_trees(street).await?;

        let by_state = self.by_state.report(&trees)?;
        let by_height = self.by_height.report(&trees)?;
        let by_crown = self.by_crown.report(&trees)?;
        let by_girth = self.by_girth.report(&trees)?;

        Ok(StreetReport {
            street: street.to_string(),
            total: trees.len(),
            area: self.area.report(&trees)?,
            states: by_state,
            heights: by_height,
            crowns: by_crown,
            girths: by_girth,
            species: self.by_species.report(&trees)?,
        })
    }

    async fn find_trees(&self, street: &str) -> Result<Vec<TreeRecord>> {
        let mut trees: Vec<TreeRecord> = vec![];
        let substring = street.to_lowercase();

        for tree in self.trees.all().await? {
            if tree.state == "gone" {
                continue;
            }

            if let Some(address) = &tree.address {
                if address.to_lowercase().contains(&substring) {
                    trees.push(tree);
                }
            }
        }

        Ok(trees)
    }
}

impl Locatable for GetStreetReportHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
            area: locator.get::<TreesAreaReporter>()?,
            by_state: locator.get::<TreesByStateReporter>()?,
            by_height: locator.get::<TreesByHeightReporter>()?,
            by_crown: locator.get::<TreesByCrownReporter>()?,
            by_girth: locator.get::<TreesByGirthReporter>()?,
            by_species: locator.get::<TreesBySpeciesReporter>()?,
        })
    }
}
