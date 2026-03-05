//! Handler for new file uploads.
//!
//! Receives file contents and information on who uploaded it.
//! Creates a new records in the `uploads` table for this file,
//! saves the file in the configured file storage (local or remote).
//! Returns new file id.

use super::models::Upload;
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
    pub async fn create_upload_ticket(
        &self,
        user_id: u64,
        size: u64,
        remote_addr: String,
        user_agent: String,
    ) -> Result<FileUploadResponse> {
        let file_id = get_unique_id()?;
        let url = self.storage.create_upload_url(file_id).await?;

        self.uploads
            .add(&Upload {
                id: file_id,
                added_by: user_id,
                added_at: get_timestamp(),
                size,
                uploaded_at: None,
            })
            .await?;

        info!(
            "Created upload ticket for user {user_id}, id={file_id}, size={size}; addr={remote_addr} agent={user_agent}"
        );

        Ok(FileUploadResponse {
            id: file_id.to_string(),
            url: Some(url),
        })
    }

    pub async fn finish_upload(&self, id: u64) -> Result<()> {
        self.uploads.finish(id, get_timestamp()).await?;
        info!("Upload {id} finished");
        Ok(())
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
