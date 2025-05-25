//! Add a photo to a tree (message handler).
//!
//! This message is sent whenever a user adds a file to a tree as a photo.

use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::*;
use log::{debug, error, info};
use std::sync::Arc;

const SMALL_SIZE: u32 = 1000;
const LARGE_SIZE: u32 = 2000;

pub struct AddPhotoHandler {
    files: Arc<FileRepository>,
    uploads: Arc<UploadRepository>,
    storage: Arc<dyn FileStorageInterface>,
    thumbnailer: Arc<ThumbnailerService>,
    trees: Arc<TreeRepository>,
}

impl AddPhotoHandler {
    pub async fn handle(&self, tree_id: u64, file_id: u64) -> Result<()> {
        let source = self
            .uploads
            .get(file_id)
            .await
            .inspect_err(|e| {
                error!("Could not find source file {}: {:?}", file_id, e);
            })?
            .ok_or(Error::FileNotFound)?;

        let body = self.storage.read_file(source.id).await.inspect_err(|e| {
            error!("Could not read source file {}: {:?}", source.id, e);
        })?;

        let small = self.thumbnailer.resize(&body, SMALL_SIZE)?;
        let small_id = self.write_file(&small).await?;

        let large = self.thumbnailer.resize(&body, LARGE_SIZE)?;
        let large_id = self.write_file(&large).await?;

        let rec = FileRecord {
            id: source.id,
            tree_id,
            small_id,
            large_id,

            added_at: source.added_at,
            added_by: source.added_by,

            ..Default::default()
        };

        self.files.add(&rec).await.inspect_err(|e| {
            error!("Could not add file {}: {:?}", file_id, e);
        })?;

        self.uploads.delete(&source).await?;

        self.update_thumbnail(tree_id, small_id, source.added_by)
            .await?;

        info!(
            "File {} successfully processed, added to tree {}.",
            source.id, tree_id
        );

        Ok(())
    }

    async fn update_thumbnail(&self, tree_id: u64, file_id: u64, user_id: u64) -> Result<()> {
        if let Some(tree) = self.trees.get(tree_id).await? {
            if tree.thumbnail_id.is_some() {
                debug!("Tree {tree_id} has a thumbnail, not updating.");
                return Ok(());
            }

            self.trees
                .update_thumbnail(tree_id, file_id, user_id)
                .await?;
        }

        Ok(())
    }

    /**
     * Writes the binary data to a new file, returns its id.
     *
     * Note that the id is only allocated, file info is not yet
     * stored in the database.
     */
    async fn write_file(&self, data: &[u8]) -> Result<u64> {
        let id = get_unique_id()?;

        match self.storage.write_file(id, data).await {
            Ok(_) => Ok(id),
            Err(_) => Err(Error::FileUpload),
        }
    }
}

impl Locatable for AddPhotoHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            files: locator.get::<FileRepository>()?,
            uploads: locator.get::<UploadRepository>()?,
            storage: locator.get::<FileStorageSelector>()?.driver(),
            thumbnailer: locator.get::<ThumbnailerService>()?,
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
