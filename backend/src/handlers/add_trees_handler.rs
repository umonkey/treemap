use crate::services::*;
use crate::types::*;
use crate::utils::{fix_circumference, get_timestamp, get_unique_id};
use log::debug;
use std::sync::Arc;

pub struct AddTreesHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl AddTreesHandler {
    pub fn new(db: Arc<dyn DatabaseInterface>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, req: AddTreeRequest) -> Result<Vec<TreeRecord>> {
        let now = get_timestamp();

        let mut trees: Vec<TreeRecord> = Vec::new();

        for point in req.points {
            let tree = TreeRecord {
                id: get_unique_id()?,
                osm_id: None,
                lat: point.lat,
                lon: point.lon,
                species: req.species.clone(),
                notes: req.notes.clone(),
                height: req.height,
                circumference: fix_circumference(req.circumference),
                diameter: req.diameter,
                state: req.state.to_string(),
                added_at: now,
                updated_at: now,
                added_by: req.user_id,
                thumbnail_id: None,
                year: req.year,
                address: req.address.clone(),
            };

            debug!(
                "Adding tree at ({}, {}) with species '{}'.",
                tree.lat, tree.lon, tree.species
            );

            self.db.add_tree(&tree).await?;

            trees.push(tree);
        }

        Ok(trees)
    }
}

impl Locatable for AddTreesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<SqliteDatabase>()?;
        Ok(Self::new(db))
    }
}
