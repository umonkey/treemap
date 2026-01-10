use super::models::File;
use super::repository::FileRepository;
use crate::actions::tree::AddFileRequest;
use crate::common::database::repositories::UserRepository;
use crate::infra::queue::Queue;
use crate::infra::storage::FileStorage;
use crate::services::{Locatable, Locator, ThumbnailerService};
use crate::types::{Error, ResizeImageMessage, Result};
use crate::utils::{get_timestamp, get_unique_id};
use log::{debug, info};
use std::sync::Arc;

pub struct FileService {
    queue: Arc<Queue>,
    storage: Arc<FileStorage>,
    files: Arc<FileRepository>,
    users: Arc<UserRepository>,
    thumbnailer: Arc<ThumbnailerService>,
}

impl FileService {
    pub async fn add_file(&self, req: AddFileRequest) -> Result<File> {
        info!(
            "Received {} bytes from user {}, storing; addr={} agent={}",
            req.file.len(),
            req.user_id,
            req.remote_addr,
            req.user_agent,
        );

        self.thumbnailer.validate(&req.file)?;

        let id = self.write_file(&req.file).await?;

        debug!("Going to add file {id} to the database.");

        let file_record = File {
            id,
            tree_id: req.tree_id,
            added_at: get_timestamp(),
            added_by: req.user_id,
            ..Default::default()
        };

        self.files.add(&file_record).await?;
        self.schedule_file_processing(id).await?;

        self.users.increment_files_count(req.user_id, 1).await?;

        info!(
            "User {} added file {} to tree {}",
            req.user_id, id, req.tree_id
        );

        Ok(file_record)
    }

    pub async fn delete_file(&self, user_id: u64, file_id: u64) -> Result<()> {
        let file = self.files.get(file_id).await?.ok_or(Error::FileNotFound)?;

        if file.deleted_at.is_some() {
            return Err(Error::FileNotFound);
        }

        self.files
            .update(&File {
                deleted_at: Some(get_timestamp()),
                deleted_by: Some(user_id),
                ..file
            })
            .await?;

        self.users.increment_files_count(file.added_by, -1).await?;

        info!("File {file_id} deleted by {user_id}.");

        Ok(())
    }

    pub async fn get_file_status(&self, id: u64) -> Result<File> {
        self.files.get(id).await?.ok_or(Error::FileNotFound)
    }

    pub async fn get_file(&self, id: u64) -> Result<Vec<u8>> {
        debug!("Reading file {id} from storage.");
        self.storage.read_file(id).await
    }

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
            storage: locator.get::<FileStorage>()?,
            queue: locator.get::<Queue>()?,
            files: locator.get::<FileRepository>()?,
            users: locator.get::<UserRepository>()?,
            thumbnailer: locator.get::<ThumbnailerService>()?,
        })
    }
}
