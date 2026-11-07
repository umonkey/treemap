//! File ticket creation, processing, and legacy reads.

use super::models::{File, FileKind, FileStatus, FileVariant};
use super::repository::FileRepository;
use crate::infra::queue::Queue;
use crate::infra::storage::FileBucket;
use crate::services::queue_consumer::ProcessFileMessage;
use crate::services::{Context, Injectable, ThumbnailerService};
use crate::types::{Error, Result};
use crate::utils::{get_timestamp, get_unique_id};
use log::{debug, info};
use std::sync::Arc;

const SMALL_SIZE: u32 = 480;
const LARGE_SIZE: u32 = 2000;

pub struct FileService {
    files: Arc<FileRepository>,
    storage: Arc<FileBucket>,
    queue: Arc<Queue>,
    thumbnailer: Arc<ThumbnailerService>,
}

impl FileService {
    /// Create a pending file record and a presigned upload URL.
    pub async fn create_ticket(
        &self,
        user_id: u64,
        size: u64,
        content_type: &str,
        kind: FileKind,
    ) -> Result<(u64, String)> {
        let content_type = content_type.trim().to_lowercase();

        if content_type.contains("heic") || content_type.contains("heif") {
            return Err(Error::BadRequestMessage(
                "HEIC/HEIF images are not supported. Please upload a JPEG or PNG.".to_string(),
            ));
        }

        let id = get_unique_id()?;
        let extension = extension_for_content_type(&content_type);
        let key = format!("files/{id}/original.{extension}");
        let url = self.storage.create_upload_url_at(&key).await?;

        self.files
            .add(&File {
                id,
                kind,
                status: FileStatus::Pending,
                is_visible: true,
                created_at: get_timestamp(),
                created_by: user_id,
                ..Default::default()
            })
            .await?;

        self.files
            .add_variant(&FileVariant {
                file_id: id,
                name: "original".to_string(),
                key,
                mime_type: content_type,
                size_bytes: size,
                width: None,
                height: None,
                created_at: get_timestamp(),
            })
            .await?;

        info!("Created file ticket {id} for user {user_id}, size={size}.");

        Ok((id, url))
    }

    /// Mark an upload as complete and enqueue processing.
    pub async fn finish(&self, user_id: u64, id: u64) -> Result<()> {
        let file = self.files.get(id).await?.ok_or(Error::FileNotFound)?;

        if file.is_deleted() {
            return Err(Error::FileNotFound);
        }

        if file.created_by != user_id {
            return Err(Error::AccessDenied);
        }

        if file.status == FileStatus::Ready || file.status == FileStatus::Processing {
            return Ok(());
        }

        self.files
            .update_status(id, FileStatus::Processing, None)
            .await?;

        let message = ProcessFileMessage { id };
        self.queue.push(&message.encode()).await?;

        info!("File {id} finished uploading, processing queued.");

        Ok(())
    }

    pub async fn get(&self, id: u64) -> Result<Option<File>> {
        self.files.get(id).await
    }

    /// Soft-delete a file owned by the caller.
    pub async fn delete(&self, user_id: u64, id: u64) -> Result<()> {
        let file = self.files.get(id).await?.ok_or(Error::FileNotFound)?;

        if file.is_deleted() {
            return Err(Error::FileNotFound);
        }

        if file.created_by != user_id {
            return Err(Error::AccessDenied);
        }

        self.files
            .mark_deleted(id, user_id, get_timestamp())
            .await?;

        info!("File {id} deleted by user {user_id}.");

        Ok(())
    }

    /// Read a file through the legacy `/v1/files/{id}.jpg` route.
    ///
    /// Resolution order: `files_fallback`, then the `small` variant of a file
    /// with this id, then the bare legacy storage key.
    pub async fn read_legacy(&self, id: u64) -> Result<(Vec<u8>, String)> {
        let old_key = id.to_string();

        if let Some(fallback) = self.files.get_fallback(&old_key).await? {
            if let Some(variant) = self
                .files
                .get_variant(fallback.file_id, &fallback.variant)
                .await?
            {
                let data = self.storage.read_file_at(&variant.key).await?;
                return Ok((data, variant.mime_type));
            }
        }

        if let Some(variant) = self.files.get_variant(id, "small").await? {
            let data = self.storage.read_file_at(&variant.key).await?;
            return Ok((data, variant.mime_type));
        }

        let data = self.storage.read_file(id).await?;

        Ok((data, "image/jpeg".to_string()))
    }

    /// Process an uploaded original into `small` and `large` JPEG variants.
    pub async fn process(&self, id: u64) -> Result<()> {
        let file = match self.files.get(id).await? {
            Some(file) => file,
            None => return Ok(()),
        };

        if file.status == FileStatus::Ready || file.is_deleted() {
            return Ok(());
        }

        let original = self
            .files
            .get_variant(id, "original")
            .await?
            .ok_or(Error::FileNotFound)?;

        let body = self.storage.read_file_at(&original.key).await?;

        match self.process_body(id, &body).await {
            Ok(()) => {
                self.files
                    .update_status(id, FileStatus::Ready, Some(get_timestamp()))
                    .await?;

                info!("File {id} processed and ready.");

                Ok(())
            }

            Err(error) => {
                self.files
                    .update_status(id, FileStatus::Failed, None)
                    .await?;

                Err(error)
            }
        }
    }

    async fn process_body(&self, id: u64, body: &[u8]) -> Result<()> {
        let (small, small_width, small_height) = self.thumbnailer.variant(body, SMALL_SIZE)?;
        let (large, large_width, large_height) = self.thumbnailer.variant(body, LARGE_SIZE)?;

        let original = self.thumbnailer.decode(body)?;

        let small_key = format!("files/{id}/small.jpg");
        let large_key = format!("files/{id}/large.jpg");

        self.storage.write_file_at(&small_key, &small, true).await?;

        self.storage.write_file_at(&large_key, &large, true).await?;

        let now = get_timestamp();

        self.files
            .add_variant(&FileVariant {
                file_id: id,
                name: "small".to_string(),
                key: small_key,
                mime_type: "image/jpeg".to_string(),
                size_bytes: small.len() as u64,
                width: Some(small_width),
                height: Some(small_height),
                created_at: now,
            })
            .await?;

        self.files
            .add_variant(&FileVariant {
                file_id: id,
                name: "large".to_string(),
                key: large_key,
                mime_type: "image/jpeg".to_string(),
                size_bytes: large.len() as u64,
                width: Some(large_width),
                height: Some(large_height),
                created_at: now,
            })
            .await?;

        self.files
            .update_variant_dimensions(id, "original", original.width(), original.height())
            .await?;

        debug!("File {id} variants written.");

        Ok(())
    }
}

impl Injectable for FileService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            files: Arc::new(ctx.build::<FileRepository>()?),
            storage: ctx.storage(),
            queue: ctx.queue(),
            thumbnailer: Arc::new(ctx.build::<ThumbnailerService>()?),
        })
    }
}

fn extension_for_content_type(content_type: &str) -> &'static str {
    match content_type {
        "image/jpeg" | "image/jpg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        "image/gif" => "gif",
        "video/mp4" => "mp4",
        "video/quicktime" => "mov",
        _ => "bin",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::{AppState, ContextExt};

    async fn setup() -> Arc<FileService> {
        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        Arc::new(
            state
                .build::<FileService>()
                .expect("Error creating file service."),
        )
    }

    #[tokio::test]
    async fn test_rejects_heic() {
        let service = setup().await;

        let result = service
            .create_ticket(1, 100, "image/heic", FileKind::Photo)
            .await;

        assert!(matches!(result, Err(Error::BadRequestMessage(_))));
    }
}
