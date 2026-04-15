//! Returns street addresses that contain a substring.

use super::schemas::*;
use super::trees_area::TreesAreaReporter;
use super::trees_by_crown::TreesByCrownReporter;
use super::trees_by_girth::TreesByGirthReporter;
use super::trees_by_height::TreesByHeightReporter;
use super::trees_by_species::TreesBySpeciesReporter;
use super::trees_by_state::TreesByStateReporter;
use crate::domain::tree::{Tree, TreeRepository};
use crate::infra::database::{Database, Value};
use crate::services::{Context, Injectable};
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
        let pattern = format!("%{}%", query.trim().to_lowercase());

        let rows = self
            .db
            .fetch_sql(
                "SELECT DISTINCT address FROM trees WHERE state <> 'gone' AND address LIKE ?1 ORDER BY address LIMIT 10",
                &[Value::from(pattern)],
            )
            .await?;

        let mut streets: Vec<String> = Vec::new();

        for row in rows {
            streets.push(row.require_string("address")?);
        }

        Ok(streets)
    }

    pub async fn get_report(&self, street: &str) -> Result<StreetReport> {
        let trees = self.find_trees(street).await?;

        let by_state = self.by_state.report(&trees)?;
        let by_height = self.by_height.report(&trees)?;
        let by_crown = self.by_crown.report(&trees)?;
        let by_girth = self.by_girth.report(&trees)?;

        let (total_shade, average_shade) = self.area.report(&trees)?;

        Ok(StreetReport {
            street: street.to_string(),
            total: trees.len(),
            existing: self.count_existing(&trees),
            total_shade,
            average_shade,
            states: by_state,
            heights: by_height,
            crowns: by_crown,
            girths: by_girth,
            species: self.by_species.report(&trees)?,
        })
    }

    pub async fn get_trees_on_street(&self, street: &str) -> Result<Vec<Tree>> {
        let mut trees: Vec<Tree> = vec![];
        let substring = street.to_lowercase();

        for tree in self.trees.all().await? {
            if tree.species.to_lowercase() == "error" {
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

    async fn find_trees(&self, street: &str) -> Result<Vec<Tree>> {
        let mut trees: Vec<Tree> = vec![];
        let substring = street.to_lowercase();

        for tree in self.trees.all().await? {
            if tree.state == "replaced" || tree.species.to_lowercase() == "error" {
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

    fn count_existing(&self, trees: &[Tree]) -> usize {
        trees.iter().filter(|t| t.is_existing()).count()
    }
}

impl Injectable for StreetService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            db: ctx.database(),
            trees: Arc::new(ctx.build::<TreeRepository>()?),
            area: TreesAreaReporter::new(),
            by_state: TreesByStateReporter::new(),
            by_height: TreesByHeightReporter::new(),
            by_crown: TreesByCrownReporter::new(),
            by_girth: TreesByGirthReporter::new(),
            by_species: TreesBySpeciesReporter::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::AppState;
    use crate::services::ContextExt;
    use log::debug;

    async fn setup() -> StreetService {
        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        let state = AppState::new().await.expect("Error creating app state.");

        state
            .build::<StreetService>()
            .expect("Error creating StreetService")
    }

    #[tokio::test]
    async fn test_search() {
        let service = setup().await;

        service
            .db
            .execute("DELETE FROM trees")
            .await
            .expect("Error clearing trees.");

        service
            .db
            .execute("INSERT INTO trees (id, lat, lon, species, address, added_at, updated_at, updated_by, added_by) VALUES (1, 40.1, 44.1, 'Birch', 'Northern Ave', 0, 0, 1, 1)")
            .await
            .expect("Error adding tree.");

        let streets = service
            .search(" northern ")
            .await
            .expect("Error finding streets.");

        assert_eq!(streets.len(), 1);
        assert_eq!(streets[0], "Northern Ave");
    }
}
