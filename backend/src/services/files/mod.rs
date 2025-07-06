use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use log::{debug, info};
use std::sync::Arc;

pub struct FileService {
    queue: Arc<QueueService>,
    storage: Arc<dyn FileStorageInterface>,
    files: Arc<FileRepository>,
}

impl FileService {
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
            ..Default::default()
        };

        self.files.add(&file_record).await?;

        self.schedule_file_processing(id).await?;

        info!(
            "User {} added file {} to tree {}",
            req.user_id, id, req.tree_id
        );

        Ok(file_record)
    }

    /**
     * Read file contents from the local file system or remote.
     */
    pub async fn get_file(&self, id: u64) -> Result<Vec<u8>> {
        debug!("Reading file {} from storage.", id);
        self.storage.read_file(id).await
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

impl Locatable for FileService {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            storage: locator.get::<FileStorageSelector>()?.driver(),
            queue: locator.get::<QueueService>()?,
            files: locator.get::<FileRepository>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env;
    use std::path::Path;

    async fn setup() -> Result<Arc<FileService>> {
        env::set_var("FILE_FOLDER", "var/test-files");
        env::set_var("TREEMAP_SQLITE_PATH", ":memory:");
        env::set_var("AWS_ACCESS_KEY_ID", "");

        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        let locator = Locator::new();
        locator.get::<FileService>()
    }

    #[tokio::test]
    async fn test_add_file() {
        let service = setup().await.expect("Failed to create FileService");
        let body = include_bytes!("test/tree.jpg").to_vec();

        let req = AddFileRequest {
            user_id: 1,
            tree_id: 2,
            file: body,
            ..Default::default()
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
