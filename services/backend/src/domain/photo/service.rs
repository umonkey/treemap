//! Add a photo to a tree (message handler).
//!
//! This message is sent whenever a user adds a file to a tree as a photo.

use crate::domain::file::FileRepository;
use crate::domain::tree_file::TreeFileService;
use crate::services::{Context, Injectable};
use crate::types::Result;
use log::{info, warn};
use std::sync::Arc;

pub struct PhotoService {
    files: Arc<FileRepository>,
    tree_files: Arc<TreeFileService>,
}

impl PhotoService {
    /// Attach a universal file to a tree.
    ///
    /// A missing file row is not an error: the message would otherwise be
    /// delayed and retried forever.
    pub async fn add_tree_photo(&self, tree_id: u64, file_id: u64) -> Result<()> {
        let file = self.files.get(file_id).await?;

        match file {
            Some(file) => {
                self.tree_files
                    .attach(tree_id, file.created_by, vec![file_id])
                    .await?;

                info!("File {file_id} attached to tree {tree_id}.");

                Ok(())
            }

            None => {
                warn!("File {file_id} not found; dropping AddPhoto message for tree {tree_id}.");

                Ok(())
            }
        }
    }
}

impl Injectable for PhotoService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            files: Arc::new(ctx.build::<FileRepository>()?),
            tree_files: Arc::new(ctx.build::<TreeFileService>()?),
        })
    }
}
