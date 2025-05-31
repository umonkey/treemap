//! Handles a request to add one or more trees.
//!
//! Receives information on the trees, a bunch of points, and some files to attach.
//! The files are processed in the background.

use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::{fix_circumference, get_timestamp, get_unique_id};
use log::{debug, error, info};
use std::sync::Arc;

pub struct AddTreesHandler {
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
    queue: Arc<QueueService>,
}

impl AddTreesHandler {
    pub async fn handle(&self, req: AddTreeRequest) -> Result<Vec<TreeRecord>> {
        let now = get_timestamp();

        let mut trees: Vec<TreeRecord> = Vec::new();

        for point in &req.points {
            let tree = self.add_tree(point.lat, point.lon, &req, now).await?;
            trees.push(tree);
        }

        Ok(trees)
    }

    // Add a single tree.
    async fn add_tree(
        &self,
        lat: f64,
        lon: f64,
        req: &AddTreeRequest,
        now: u64,
    ) -> Result<TreeRecord> {
        let tree = TreeRecord {
            id: get_unique_id()?,
            lat,
            lon,
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
            lat, lon, tree.species
        );

        self.trees.add(&tree).await?;
        self.schedule_address_update(tree.id).await?;
        self.schedule_files(tree.id, req.files.clone()).await?;
        self.users.increment_tree_count(req.user_id).await?;

        Ok(tree)
    }

    async fn schedule_address_update(&self, tree_id: u64) -> Result<()> {
        self.queue
            .push(QueueCommand::UpdateTreeAddress(UpdateTreeAddressMessage {
                id: tree_id,
            }))
            .await?;

        info!("Scheduled address update for tree {}", tree_id);

        Ok(())
    }

    // Send files for attaching to the new tree.
    async fn schedule_files(&self, tree_id: u64, files: Vec<String>) -> Result<()> {
        for file in files {
            match file.parse::<u64>() {
                Ok(file_id) => {
                    info!("Scheduling file {} for tree {}", file_id, tree_id);

                    self.queue
                        .push(QueueCommand::AddPhoto(AddPhotoMessage { tree_id, file_id }))
                        .await?;
                }

                Err(e) => {
                    error!(
                        "Error parsing file ID '{}': {} -- cannot attach to tree {}.",
                        file, e, tree_id
                    );
                }
            }
        }

        Ok(())
    }
}

impl Locatable for AddTreesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
            queue: locator.get::<QueueService>()?,
        })
    }
}
