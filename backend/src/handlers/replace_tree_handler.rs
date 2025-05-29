use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::{fix_circumference, get_timestamp, get_unique_id};
use log::{error, info};
use std::sync::Arc;

pub struct ReplaceTreeHandler {
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
    queue: Arc<QueueService>,
}

impl ReplaceTreeHandler {
    pub async fn handle(&self, req: ReplaceTreeRequest) -> Result<Vec<TreeRecord>> {
        let now = get_timestamp();

        let old = self.trees.get(req.id).await?.ok_or(Error::TreeNotFound)?;

        let new = TreeRecord {
            id: get_unique_id()?,
            lat: old.lat,
            lon: old.lon,
            species: req.species.clone(),
            notes: req.notes.clone(),
            height: req.height,
            circumference: fix_circumference(req.circumference),
            diameter: req.diameter,
            state: req.state.to_string(),
            added_at: now,
            added_by: req.user_id,
            updated_at: now,
            year: req.year,
            address: old.address.clone(),
            replaces: Some(old.id),
            ..Default::default()
        };

        self.trees.add(&new).await?;
        self.trees.replace(old.id, new.id, req.user_id).await?;
        self.schedule_files(new.id, req.files.clone()).await?;
        self.users.increment_tree_count(req.user_id).await?;

        info!(
            "Tree {} added as a replacement for tree {} by {}",
            new.id, req.id, req.user_id
        );

        let trees: Vec<TreeRecord> = vec![
            TreeRecord {
                state: "gone".to_string(),
                replaced_by: Some(new.id),
                ..old
            },
            new,
        ];

        Ok(trees)
    }

    // Send files for attaching to the new tree.
    async fn schedule_files(&self, tree_id: u64, files: Vec<String>) -> Result<()> {
        for file in files {
            match file.parse::<u64>() {
                Ok(file_id) => {
                    info!("Scheduling file {} for tree {}", file_id, tree_id);

                    let message = AddPhotoMessage { tree_id, file_id };
                    self.queue.push(&message.encode()).await?;
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

impl Locatable for ReplaceTreeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
            queue: locator.get::<QueueService>()?,
        })
    }
}
