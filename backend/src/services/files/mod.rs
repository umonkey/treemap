use log::{debug, error, info};
use std::sync::Arc;
use tokio::fs;

use crate::services::{Database, QueueService, ThumbnailerService};
use crate::types::{AddFileRequest, Error, FileRecord, ResizeImageMessage, Result};
use crate::utils::{get_file_folder, get_timestamp, get_unique_id};

const SMALL_SIZE: u32 = 1000;
const LARGE_SIZE: u32 = 2000;

pub struct FileService {
    db: Arc<dyn Database>,
    folder: String,
    thumbnailer: ThumbnailerService,
    queue: QueueService,
}

impl FileService {
    pub fn init(db: &Arc<dyn Database>) -> Result<Self> {
        let folder = get_file_folder();

        Ok(Self {
            db: db.clone(),
            folder,
            queue: QueueService::init(db)?,
            thumbnailer: ThumbnailerService::init(),
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
        match self.db.get_file(file_id).await {
            Ok(Some(file)) => {
                let body = self.get_file(file.id).await?;

                let small = self.thumbnailer.resize(&body, SMALL_SIZE)?;
                let small_id = self.write_file(&small).await?;

                let large = self.thumbnailer.resize(&body, LARGE_SIZE)?;
                let large_id = self.write_file(&large).await?;

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

    pub async fn find_files_by_tree(&self, tree_id: u64) -> Result<Vec<FileRecord>> {
        self.db.find_files_by_tree(tree_id).await
    }

    pub async fn get_file(&self, id: u64) -> Result<Vec<u8>> {
        let file_path = format!("{}/{}", self.folder, id);

        match fs::read(file_path).await {
            Ok(b) => Ok(b),

            Err(e) => {
                error!("Error reading file: {:?}", e);
                Err(Error::FileDownload)
            }
        }
    }

    /**
     * Writes the binary data to a new file, returns its id.
     *
     * Note that the id is only allocated, file info is not yet
     * stored in the database.
     */
    async fn write_file(&self, data: &Vec<u8>) -> Result<u64> {
        let id = get_unique_id()?;
        let file_path = format!("{}/{}", self.folder, id);

        debug!("Writing file to: {}", file_path);

        match fs::create_dir_all(&self.folder).await {
            Ok(()) => (),

            Err(e) => {
                error!("Error creating folder: {:?}", e);
                return Err(Error::FileUpload);
            }
        };

        match fs::write(file_path, data).await {
            Ok(()) => Ok(id),

            Err(e) => {
                error!("Error writing file: {:?}", e);
                Err(Error::FileUpload)
            }
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
    use crate::services::get_database;
    use env_logger;
    use std::env;
    use std::path::Path;

    async fn setup() -> Result<FileService> {
        env::set_var("FILE_FOLDER", "var/test-files");
        env::set_var("TREEMAP_SQLITE_PATH", ":memory:");

        if let Err(_) = env_logger::try_init() {
            debug!("env_logger already initialized.");
        };

        let db = get_database().await.expect("Error creating the database");

        FileService::init(&db)
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
