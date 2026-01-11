//! Handler for new file uploads.
//!
//! Receives file contents and information on who uploaded it.
//! Creates a new records in the `uploads` table for this file,
//! saves the file in the configured file storage (local or remote).
//! Returns new file id.

use super::repository::UploadRepository;
use crate::actions::tree::FileUploadResponse;
use crate::infra::storage::FileStorage;
use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use log::info;
use std::sync::Arc;

pub struct UploadService {
    uploads: Arc<UploadRepository>,
    storage: Arc<FileStorage>,
}

impl UploadService {
    pub async fn upload_file(&self, req: FileUploadRequest) -> Result<FileUploadResponse> {
        let file_id = get_unique_id()?;

        self.storage.write_file(file_id, &req.file).await?;

        self.uploads
            .add(&UploadRecord {
                id: file_id,
                added_by: req.user_id,
                added_at: get_timestamp(),
                size: req.file.len() as u64,
            })
            .await?;

        info!(
            "Received {} bytes from user {}, id={}; addr={} agent={}",
            req.file.len(),
            req.user_id,
            file_id,
            req.remote_addr,
            req.user_agent,
        );

        Ok(FileUploadResponse::from_id(file_id))
    }
}

impl Locatable for UploadService {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            uploads: locator.get::<UploadRepository>()?,
            storage: locator.get::<FileStorage>()?,
        })
    }
}
