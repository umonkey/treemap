use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::get_timestamp;
use log::info;
use std::sync::Arc;

pub struct DeleteFileHandler {
    files: Arc<FileRepository>,
    users: Arc<UserRepository>,
}

impl DeleteFileHandler {
    pub async fn handle(&self, req: DeleteFileRequest) -> Result<()> {
        let file = self
            .files
            .get(req.file_id)
            .await?
            .ok_or(Error::FileNotFound)?;

        if file.deleted_at.is_some() {
            return Err(Error::FileNotFound);
        }

        self.files
            .update(&FileRecord {
                deleted_at: Some(get_timestamp()),
                deleted_by: Some(req.user_id),
                ..file
            })
            .await?;

        self.users.increment_files_count(file.added_by, -1).await?;

        info!("File {} deleted by {}.", file.id, req.user_id);

        Ok(())
    }
}

impl Locatable for DeleteFileHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            files: locator.get::<FileRepository>()?,
            users: locator.get::<UserRepository>()?,
        })
    }
}
