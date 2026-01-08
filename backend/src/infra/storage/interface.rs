use super::base::FileStorageInterface;
use super::local_storage::LocalFileStorage;
use super::s3_storage::S3FileStorage;
use crate::config::Config;
use crate::services::{Locatable, Locator};
use crate::types::*;
use log::error;
use std::sync::Arc;

pub struct FileStorage {
    client: Arc<dyn FileStorageInterface>,
}

impl FileStorage {
    pub fn new(client: Arc<dyn FileStorageInterface>) -> Self {
        Self {
            client: client.clone(),
        }
    }
}

impl FileStorage {
    pub async fn write_file(&self, id: u64, data: &[u8]) -> Result<()> {
        self.client.write_file(id, data).await
    }

    pub async fn read_file(&self, id: u64) -> Result<Vec<u8>> {
        self.client.read_file(id).await
    }
}

impl Locatable for FileStorage {
    fn create(locator: &Locator) -> Result<Self> {
        let config = locator.get::<Config>()?;

        if config.file_storage == "s3" {
            let client = locator.get::<S3FileStorage>()?;
            return Ok(Self::new(client));
        }

        if config.file_storage == "local" {
            let client = locator.get::<LocalFileStorage>()?;
            return Ok(Self::new(client));
        }

        error!("Unsupported file storage: {}", config.file_storage);
        Err(Error::Config)
    }
}
