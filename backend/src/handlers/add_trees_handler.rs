use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::{fix_circumference, get_timestamp, get_unique_id};
use log::{debug, info};
use std::sync::Arc;

pub struct AddTreesHandler {
    trees: Arc<TreeRepository>,
    queue: Arc<QueueService>,
}

impl AddTreesHandler {
    pub async fn handle(&self, req: AddTreeRequest) -> Result<Vec<TreeRecord>> {
        let now = get_timestamp();

        let mut trees: Vec<TreeRecord> = Vec::new();

        for point in req.points {
            let tree = TreeRecord {
                id: get_unique_id()?,
                lat: point.lat,
                lon: point.lon,
                species: req.species.clone(),
                notes: req.notes.clone(),
                height: req.height,
                circumference: fix_circumference(req.circumference),
                diameter: req.diameter,
                state: req.state.to_string(),
                added_at: now,
                added_by: req.user_id,
                updated_at: now,
                thumbnail_id: None,
                year: req.year,
                address: req.address.clone(),
                ..Default::default()
            };

            debug!(
                "Adding tree at ({}, {}) with species '{}'.",
                tree.lat, tree.lon, tree.species
            );

            self.trees.add(&tree).await?;
            self.schedule_address_update(tree.id).await?;

            trees.push(tree);
        }

        Ok(trees)
    }

    async fn schedule_address_update(&self, tree_id: u64) -> Result<()> {
        let msg = UpdateTreeAddressMessage { id: tree_id };
        self.queue.push(&msg.encode()).await?;

        info!("Scheduled address update for tree {}", tree_id);

        Ok(())
    }
}

impl Locatable for AddTreesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let trees = locator.get::<TreeRepository>()?;
        let queue = locator.get::<QueueService>()?;
        Ok(Self { trees, queue })
    }
}
