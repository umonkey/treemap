use crate::domain::file::{File, FileRepository};
use crate::domain::tree::TreeRepository;
use crate::infra::storage::FileStorage;
use crate::services::{Locatable, Locator, ThumbnailerService};
use crate::types::{Error, Result};
use crate::utils::get_unique_id;
use log::{debug, error, info};
use std::sync::Arc;

const SMALL_SIZE: u32 = 1000;
const LARGE_SIZE: u32 = 2000;

pub struct ResizeImageHandler {
    trees: Arc<TreeRepository>,
    files: Arc<FileRepository>,
    storage: Arc<FileStorage>,
    thumbnailer: Arc<ThumbnailerService>,
}

impl ResizeImageHandler {
    pub async fn handle(&self, file_id: u64) -> Result<()> {
        debug!("Going to resize images for file {file_id}.");

        match self.files.get(file_id).await {
            Ok(Some(file)) => {
                let body = self.storage.read_file(file.id).await?;

                let small = self.thumbnailer.resize(&body, SMALL_SIZE)?;
                let small_id = self.write_file(&small).await?;

                let large = self.thumbnailer.resize(&body, LARGE_SIZE)?;
                let large_id = self.write_file(&large).await?;

                debug!("Updating file {file_id} with new image ids.");

                let updated = File {
                    small_id,
                    large_id,
                    ..file
                };

                self.files.update(&updated).await?;

                self.trees
                    .update_thumbnail(file.tree_id, small_id, file.added_by)
                    .await?;

                info!("Resized images for file {}, tree {}", file_id, file.tree_id);
                Ok(())
            }

            Ok(None) => {
                error!("File {file_id} not found, cannot resize images.");
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
}

impl Locatable for ResizeImageHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
            files: locator.get::<FileRepository>()?,
            thumbnailer: locator.get::<ThumbnailerService>()?,
            storage: locator.get::<FileStorage>()?,
        })
    }
}
