//! Add a photo to a tree (message handler).
//!
//! This message is sent whenever a user adds a file to a tree as a photo.

use crate::domain::tree::TreeRepository;
use crate::domain::tree_image::{TreeImage, TreeImageRepository};
use crate::domain::upload::UploadRepository;
use crate::infra::storage::FileStorage;
use crate::services::ThumbnailerService;
use crate::services::{Locatable, Locator};
use crate::types::{Error, Result};
use crate::utils::*;
use log::{debug, error, info};
use std::sync::Arc;

const SMALL_SIZE: u32 = 1000;
const LARGE_SIZE: u32 = 2000;

pub struct PhotoService {
    tree_images: Arc<TreeImageRepository>,
    uploads: Arc<UploadRepository>,
    storage: Arc<FileStorage>,
    thumbnailer: Arc<ThumbnailerService>,
    trees: Arc<TreeRepository>,
}

impl PhotoService {
    // Adds a photo to a tree.
    //
    // Reads the source file, resizes it to small and large veresions,
    // saves them in the file storage, and adds a tree photo record.
    pub async fn add_tree_photo(&self, tree_id: u64, file_id: u64) -> Result<()> {
        let source = self
            .uploads
            .get(file_id)
            .await
            .inspect_err(|e| {
                error!("Could not find source file {file_id}: {e:?}");
            })?
            .ok_or(Error::FileNotFound)?;

        let body = self.storage.read_file(source.id).await.inspect_err(|e| {
            error!("Could not read source file {}: {:?}", source.id, e);
        })?;

        let small = self.thumbnailer.resize(&body, SMALL_SIZE)?;
        let small_id = self.write_file(&small).await?;

        let large = self.thumbnailer.resize(&body, LARGE_SIZE)?;
        let large_id = self.write_file(&large).await?;

        let rec = TreeImage {
            id: source.id,
            tree_id,
            small_id,
            large_id,

            added_at: source.added_at,
            added_by: source.added_by,

            ..Default::default()
        };

        self.tree_images.add(&rec).await.inspect_err(|e| {
            error!("Could not add file {file_id}: {e:?}");
        })?;

        self.uploads.delete(&source).await?;

        self.update_thumbnail(tree_id, small_id, source.added_by)
            .await?;

        info!(
            "TreeImage {} successfully processed, added to tree {}.",
            source.id, tree_id
        );

        Ok(())
    }

    pub async fn resize_image(&self, file_id: u64) -> Result<()> {
        debug!("Going to resize images for file {file_id}.");

        match self.tree_images.get(file_id).await {
            Ok(Some(file)) => {
                let body = self.storage.read_file(file.id).await?;

                let small = self.thumbnailer.resize(&body, SMALL_SIZE)?;
                let small_id = self.write_file(&small).await?;

                let large = self.thumbnailer.resize(&body, LARGE_SIZE)?;
                let large_id = self.write_file(&large).await?;

                debug!("Updating file {file_id} with new image ids.");

                let updated = TreeImage {
                    small_id,
                    large_id,
                    ..file
                };

                self.tree_images.update(&updated).await?;

                self.trees
                    .update_thumbnail(file.tree_id, small_id, file.added_by)
                    .await?;

                info!("Resized images for file {}, tree {}", file_id, file.tree_id);
                Ok(())
            }

            Ok(None) => {
                error!("TreeImage {file_id} not found, cannot resize images.");
                Ok(())
            }

            Err(e) => {
                error!("Error resizing images for file {file_id}: {e:?}");
                Ok(())
            }
        }
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
}

impl Locatable for PhotoService {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            tree_images: locator.get::<TreeImageRepository>()?,
            uploads: locator.get::<UploadRepository>()?,
            storage: locator.get::<FileStorage>()?,
            thumbnailer: locator.get::<ThumbnailerService>()?,
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
