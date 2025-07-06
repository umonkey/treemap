//! Update addresses for all trees that don't have one.
//!
//! This is only executed using a dedicated CLI command.

use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::*;
use log::{error, info};
use std::sync::Arc;

const USERPIC_SIZE: u32 = 1000;

pub struct UpdateUserpicHandler {
    users: Arc<UserRepository>,
    uploads: Arc<UploadRepository>,
    storage: Arc<dyn FileStorageInterface>,
    thumbnailer: Arc<ThumbnailerService>,
}

impl UpdateUserpicHandler {
    pub async fn handle(&self, user_id: u64, file_id: u64) -> Result<()> {
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

        let small = self.thumbnailer.resize(&body, USERPIC_SIZE)?;
        let small_id = self.write_file(&small).await?;

        let userpic = format!("https://yerevan.treemaps.app/v1/files/{small_id}.jpg");
        self.users.update_userpic(user_id, &userpic).await?;

        // TODO: save the record somewhere.  Currently we just delete it
        // and we have an orphan file not listed in the database.
        self.uploads.delete(&source).await?;

        info!(
            "File {} successfully processed, added to user {}.",
            source.id, user_id
        );

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

impl Locatable for UpdateUserpicHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            users: locator.get::<UserRepository>()?,
            uploads: locator.get::<UploadRepository>()?,
            storage: locator.get::<FileStorageSelector>()?.driver(),
            thumbnailer: locator.get::<ThumbnailerService>()?,
        })
    }
}
