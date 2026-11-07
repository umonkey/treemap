//! Handler for new file uploads.
//!
//! Creates a `files` ticket and returns a presigned upload URL.

use crate::actions::tree::FileUploadResponse;
use crate::domain::file::{FileKind, FileService};
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct UploadService {
    files: Arc<FileService>,
}

impl UploadService {
    pub async fn create_upload_ticket(
        &self,
        user_id: u64,
        size: u64,
        content_type: String,
        kind: Option<String>,
        remote_addr: String,
        user_agent: String,
    ) -> Result<FileUploadResponse> {
        let kind = kind.as_deref().map(FileKind::from_str).unwrap_or_default();

        let (id, url) = self
            .files
            .create_ticket(user_id, size, &content_type, kind)
            .await?;

        info!(
            "Created upload ticket for user {user_id}, id={id}, size={size}; addr={remote_addr} agent={user_agent}"
        );

        Ok(FileUploadResponse {
            id: id.to_string(),
            url: Some(url),
        })
    }

    pub async fn finish_upload(&self, user_id: u64, id: u64) -> Result<()> {
        self.files.finish(user_id, id).await?;
        info!("Upload {id} finished by user {user_id}");
        Ok(())
    }
}

impl Injectable for UploadService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            files: Arc::new(ctx.build::<FileService>()?),
        })
    }
}
