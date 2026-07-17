use super::base::StorageDriver;
use super::local_storage::LocalStorageDriver;
use super::s3_storage::S3StorageDriver;
use crate::infra::config::Config;
use crate::infra::secrets::Secrets;
use crate::types::*;
use std::sync::Arc;

pub struct Storage {
    driver: Arc<dyn StorageDriver>,
    bucket: String,
}

impl Storage {
    pub fn new(driver: Arc<dyn StorageDriver>, bucket: String) -> Self {
        Self { driver, bucket }
    }

    pub async fn write_file(&self, path: &str, data: &[u8], public: bool) -> Result<()> {
        self.driver
            .write_file(&self.bucket, path, data, public)
            .await
    }

    pub async fn read_file(&self, path: &str) -> Result<Vec<u8>> {
        self.driver.read_file(&self.bucket, path).await
    }

    pub async fn create_upload_url(&self, path: &str) -> Result<String> {
        self.driver.create_upload_url(&self.bucket, path).await
    }
}

pub struct FileStorage {
    storage: Storage,
}

impl FileStorage {
    pub fn new(driver: Arc<dyn StorageDriver>, bucket: String) -> Self {
        Self {
            storage: Storage::new(driver, bucket),
        }
    }

    pub async fn write_file(&self, id: u64, data: &[u8], public: bool) -> Result<()> {
        self.storage.write_file(&id.to_string(), data, public).await
    }

    pub async fn read_file(&self, id: u64) -> Result<Vec<u8>> {
        self.storage.read_file(&id.to_string()).await
    }

    pub async fn create_upload_url(&self, id: u64) -> Result<String> {
        self.storage.create_upload_url(&id.to_string()).await
    }
}

pub fn create_driver(config: &Config, secrets: &Secrets) -> Result<Arc<dyn StorageDriver>> {
    if config.file_storage == "s3" {
        return Ok(Arc::new(S3StorageDriver::new(config, secrets)?));
    }

    if config.file_storage == "local" {
        return Ok(Arc::new(LocalStorageDriver::new(config)));
    }

    Err(Error::Config(format!(
        "unsupported file storage type: {}",
        config.file_storage
    )))
}
