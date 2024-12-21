use super::file_storage_interface::FileStorageInterface;
use super::local_file_storage::LocalFileStorage;
use super::s3_file_storage::S3FileStorage;
use crate::services::{Locatable, Locator};
use crate::types::*;
use std::sync::Arc;

pub struct FileStorageSelector {
    storage: Arc<dyn FileStorageInterface>,
}

impl FileStorageSelector {
    pub async fn new() -> Result<Self> {
        if let Ok(value) = S3FileStorage::new().await {
            return Ok(Self {
                storage: Arc::new(value),
            });
        }

        Ok(Self {
            storage: Arc::new(LocalFileStorage::new()),
        })
    }

    pub fn driver(&self) -> Arc<dyn FileStorageInterface> {
        self.storage.clone()
    }
}

impl Locatable for FileStorageSelector {
    fn create(_locator: &Locator) -> Result<Self> {
        futures::executor::block_on(Self::new())
    }
}
