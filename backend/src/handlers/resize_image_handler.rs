use crate::services::*;
use crate::types::*;
use crate::utils::get_unique_id;
use log::{debug, error, info};
use std::sync::Arc;

const SMALL_SIZE: u32 = 1000;
const LARGE_SIZE: u32 = 2000;

pub struct ResizeImageHandler {
    db: Arc<dyn DatabaseInterface>,
    storage: Arc<dyn FileStorageInterface>,
    thumbnailer: Arc<ThumbnailerService>,
}

impl ResizeImageHandler {
    pub async fn handle(&self, file_id: u64) -> Result<()> {
        debug!("Going to resize images for file {}.", file_id);

        match self.db.get_file(file_id).await {
            Ok(Some(file)) => {
                let body = self.storage.read_file(file.id).await?;

                let small = self.thumbnailer.resize(&body, SMALL_SIZE)?;
                let small_id = self.write_file(&small).await?;

                let large = self.thumbnailer.resize(&body, LARGE_SIZE)?;
                let large_id = self.write_file(&large).await?;

                debug!("Updating file {} with new image ids.", file_id);

                let updated = FileRecord {
                    small_id,
                    large_id,
                    ..file
                };

                self.db.update_file(&updated).await?;
                self.db
                    .update_tree_thumbnail(file.tree_id, small_id)
                    .await?;

                self.db
                    .add_tree_prop(
                        file.tree_id,
                        "thumbnail_id",
                        small_id.to_string().as_str(),
                        file.added_by,
                    )
                    .await?;

                info!("Resized images for file {}, tree {}", file_id, file.tree_id);
                Ok(())
            }

            Ok(None) => {
                error!("File {} not found, cannot resize images.", file_id);
                Ok(())
            }

            Err(e) => {
                error!("Error resizing images for file {}: {:?}", file_id, e);
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
        let db = locator.get::<PreferredDatabase>()?.driver();
        let thumbnailer = locator.get::<ThumbnailerService>()?;
        let storage = locator.get::<FileStorageSelector>()?.driver();

        Ok(Self {
            db,
            thumbnailer,
            storage,
        })
    }
}
