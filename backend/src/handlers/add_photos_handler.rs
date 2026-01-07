//! This handles a request to add files to an existing tree.
//!
//! The files were uploaded previously and their source files are
//! stored in the main storage.
//!
//! The files aren't processed immediately; instead, we send them
//! to a queue.

use crate::infra::queue::Queue;
use crate::services::*;
use crate::types::*;
use log::{error, info};
use std::sync::Arc;

pub struct AddPhotosHandler {
    queue: Arc<Queue>,
}

impl AddPhotosHandler {
    pub async fn handle(&self, req: AddPhotosRequest) -> Result<()> {
        info!(
            "Received {} files from user {} for tree {}.",
            req.files.len(),
            req.user_id,
            req.tree_id,
        );

        for file in &req.files {
            let file_id: u64 = file.parse().map_err(|e| {
                error!("Error processing file ID {file}: {e}");
                Error::BadRequest
            })?;

            self.send_file(req.tree_id, file_id).await?;
        }

        Ok(())
    }

    async fn send_file(&self, tree_id: u64, file_id: u64) -> Result<()> {
        let message = AddPhotoMessage { tree_id, file_id };
        self.queue.push(&message.encode()).await?;
        Ok(())
    }
}

impl Locatable for AddPhotosHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            queue: locator.get::<Queue>()?,
        })
    }
}
