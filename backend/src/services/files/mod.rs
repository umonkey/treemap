use log::{debug, error, info};
use std::sync::Arc;
use tokio::fs;

use crate::services::{Database, ThumbnailerService};
use crate::types::{AddFileRequest, Error, FileRecord, Result};
use crate::utils::{get_file_folder, get_timestamp, get_unique_id};

const SMALL_SIZE: u32 = 1000;
const LARGE_SIZE: u32 = 2000;

pub struct FileService {
    db: Arc<dyn Database>,
    folder: String,
    thumbnailer: ThumbnailerService,
}

impl FileService {
    pub fn init(db: &Arc<dyn Database>) -> Result<Self> {
        let folder = get_file_folder();

        Ok(Self {
            db: db.clone(),
            folder,
            thumbnailer: ThumbnailerService::init(),
        })
    }

    pub async fn add_file(&self, req: AddFileRequest) -> Result<FileRecord> {
        let id = self.write_file(&req.file).await?;

        let small = self.thumbnailer.resize(&req.file, SMALL_SIZE)?;
        let small_id = self.write_file(&small).await?;

        let large = self.thumbnailer.resize(&req.file, LARGE_SIZE)?;
        let large_id = self.write_file(&large).await?;

        debug!("Going to add file {} to the database.", id);

        let file_record = FileRecord {
            id,
            tree_id: req.tree_id,
            added_at: get_timestamp(),
            added_by: req.user_id,
            small_id,
            large_id,
        };

        self.db.add_file(&file_record).await?;
        self.db.update_tree_thumbnail(req.tree_id, large_id).await?;

        info!(
            "User {} added file {} to tree {}",
            req.user_id, id, req.tree_id
        );

        Ok(file_record)
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

        std::fs::remove_file(format!("var/test-files/{}", file.id))
            .expect("Failed to remove the file");
        std::fs::remove_file(format!("var/test-files/{}", file.small_id))
            .expect("Failed to remove the file");
        std::fs::remove_file(format!("var/test-files/{}", file.large_id))
            .expect("Failed to remove the file");
    }
}
