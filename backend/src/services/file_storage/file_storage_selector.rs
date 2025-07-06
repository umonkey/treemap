use super::file_storage_interface::FileStorageInterface;
use super::local_file_storage::LocalFileStorage;
#[allow(unused_imports)]
use super::s3_file_storage::S3FileStorage;
use crate::config::{Config, Secrets};
use crate::services::{Locatable, Locator};
use crate::types::*;
use std::sync::Arc;

pub struct FileStorageSelector {
    storage: Arc<dyn FileStorageInterface>,
}

impl FileStorageSelector {
    #[cfg(test)]
    pub async fn new(config: Arc<Config>, _secrets: Arc<Secrets>) -> Result<Self> {
        Ok(Self {
            storage: Arc::new(LocalFileStorage::new(config)),
        })
    }

    #[cfg(not(test))]
    pub async fn new(config: Arc<Config>, secrets: Arc<Secrets>) -> Result<Self> {
        if let Ok(value) = S3FileStorage::new(secrets).await {
            return Ok(Self {
                storage: Arc::new(value),
            });
        }

        Ok(Self {
            storage: Arc::new(LocalFileStorage::new(config)),
        })
    }

    pub fn driver(&self) -> Arc<dyn FileStorageInterface> {
        self.storage.clone()
    }
}

impl Locatable for FileStorageSelector {
    fn create(locator: &Locator) -> Result<Self> {
        futures::executor::block_on(Self::new(
            locator.get::<Config>()?,
            locator.get::<Secrets>()?,
        ))
    }
}
