use log::{debug, error, info};
use std::sync::Arc;

use crate::services::{Database, FileStorageInterface, QueueService, ThumbnailerService};
use crate::types::{
    AddFileRequest, Error, FileRecord, FileStatusResponse, ResizeImageMessage, Result,
};
use crate::utils::{get_timestamp, get_unique_id};

const SMALL_SIZE: u32 = 1000;
const LARGE_SIZE: u32 = 2000;

pub struct FileService {
    db: Arc<dyn Database>,
    thumbnailer: ThumbnailerService,
    queue: QueueService,
    storage: Arc<dyn FileStorageInterface>,
}

impl FileService {
    pub fn new(db: &Arc<dyn Database>, storage: &Arc<dyn FileStorageInterface>) -> Result<Self> {
        Ok(Self {
            db: db.clone(),
            queue: QueueService::new(db)?,
            storage: storage.clone(),
            thumbnailer: ThumbnailerService::new(),
        })
    }

    /**
     * Add a file to the tree.
     *
     * Image resizing is rather slow, so we offload that
     * to the background queue consumer.
     */
    pub async fn add_file(&self, req: AddFileRequest) -> Result<FileRecord> {
        let id = self.write_file(&req.file).await?;

        debug!("Going to add file {} to the database.", id);

        let file_record = FileRecord {
            id,
            tree_id: req.tree_id,
            added_at: get_timestamp(),
            added_by: req.user_id,
            small_id: 0,
            large_id: 0,
        };

        self.db.add_file(&file_record).await?;

        self.schedule_file_processing(id).await?;

        info!(
            "User {} added file {} to tree {}",
            req.user_id, id, req.tree_id
        );

        Ok(file_record)
    }

    pub async fn resize_images(&self, file_id: u64) -> Result<()> {
        debug!("Going to resize images for file {}.", file_id);

        match self.db.get_file(file_id).await {
            Ok(Some(file)) => {
                let body = self.get_file(file.id).await?;

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
                    .add_tree_prop(file.tree_id, "thumbnail_id", small_id.to_string().as_str())
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
     * Read file contents from the local file system or remote.
     */
    pub async fn get_file(&self, id: u64) -> Result<Vec<u8>> {
        debug!("Reading file {} from storage.", id);
        self.storage.read_file(id).await
    }

    pub async fn get_status(&self, id: u64) -> Result<FileStatusResponse> {
        match self.db.get_file(id).await? {
            Some(file) => Ok(FileStatusResponse::from_file(&file)),
            None => Err(Error::FileNotFound),
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

    async fn schedule_file_processing(&self, file_id: u64) -> Result<()> {
        let msg = ResizeImageMessage { id: file_id };

        self.queue.push(&msg.encode()).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::{get_database, LocalFileStorage};
    use env_logger;
    use std::env;
    use std::path::Path;

    async fn setup() -> Result<FileService> {
        env::set_var("FILE_FOLDER", "var/test-files");
        env::set_var("TREEMAP_SQLITE_PATH", ":memory:");
        env::set_var("AWS_ACCESS_KEY_ID", "");

        if let Err(_) = env_logger::try_init() {
            debug!("env_logger already initialized.");
        };

        let db = get_database().await.expect("Error creating the database");
        let storage: Arc<dyn FileStorageInterface> = Arc::new(LocalFileStorage::new());

        FileService::new(&db, &storage)
    }

    #[tokio::test]
    async fn test_add_file() {
        let service = setup().await.unwrap();
        let body = include_bytes!("test/tree.jpg").to_vec();

        let req = AddFileRequest {
            user_id: 1,
            tree_id: 2,
            file: body,
        };

        let file = service
            .add_file(req)
            .await
            .expect("Failed to store the file");
        let file_path = format!("var/test-files/{}", file.id);
        let path = Path::new(&file_path);

        assert!(path.exists());
        assert_eq!(file.small_id, 0);
        assert_eq!(file.large_id, 0);

        std::fs::remove_file(format!("var/test-files/{}", file.id))
            .expect("Failed to remove the file");
    }
}
