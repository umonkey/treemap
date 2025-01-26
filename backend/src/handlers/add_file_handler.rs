use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct AddFileHandler {
    files: Arc<FileService>,
    thumbnailer: Arc<ThumbnailerService>,
    users: Arc<UserRepository>,
}

impl AddFileHandler {
    pub async fn handle(&self, req: AddFileRequest) -> Result<FileUploadResponse> {
        let user_id = req.user_id;

        info!(
            "Received {} bytes from user {}, storing; addr={} agent={}",
            req.file.len(),
            user_id,
            req.remote_addr,
            req.user_agent,
        );

        self.thumbnailer.validate(&req.file)?;

        let file = self.files.add_file(req).await?;
        self.users.increment_files_count(user_id, 1).await?;
        Ok(FileUploadResponse::from_file(&file))
    }
}

impl Locatable for AddFileHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            files: locator.get::<FileService>()?,
            thumbnailer: locator.get::<ThumbnailerService>()?,
            users: locator.get::<UserRepository>()?,
        })
    }
}
