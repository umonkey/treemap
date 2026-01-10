//! Returns street addresses that contain a substring.

use super::schemas::*;
use super::trees_area::TreesAreaReporter;
use super::trees_by_crown::TreesByCrownReporter;
use super::trees_by_girth::TreesByGirthReporter;
use super::trees_by_height::TreesByHeightReporter;
use super::trees_by_species::TreesBySpeciesReporter;
use super::trees_by_state::TreesByStateReporter;
use crate::domain::tree::{Tree, TreeRepository};
use crate::infra::database::Database;
use crate::services::{Locatable, Locator};
use crate::types::Result;
use std::sync::Arc;

pub struct StreetService {
    db: Arc<Database>,
    trees: Arc<TreeRepository>,
    area: TreesAreaReporter,
    by_state: TreesByStateReporter,
    by_height: TreesByHeightReporter,
    by_crown: TreesByCrownReporter,
    by_girth: TreesByGirthReporter,
    by_species: TreesBySpeciesReporter,
}

impl StreetService {
    pub async fn search(&self, query: &str) -> Result<Vec<String>> {
        self.db.find_streets(query).await
    }

    pub async fn get_report(&self, street: &str) -> Result<StreetReport> {
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

    async fn find_trees(&self, street: &str) -> Result<Vec<Tree>> {
        let mut trees: Vec<Tree> = vec![];
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

impl Locatable for StreetService {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            db: locator.get::<Database>()?,
            trees: locator.get::<TreeRepository>()?,
            area: TreesAreaReporter::new(),
            by_state: TreesByStateReporter::new(),
            by_height: TreesByHeightReporter::new(),
            by_crown: TreesByCrownReporter::new(),
            by_girth: TreesByGirthReporter::new(),
            by_species: TreesBySpeciesReporter::new(),
        })
    }
}
