//! Defines wrappers for the different storage buckets.

use super::base::StorageDriver;
use crate::infra::config::Config;
use crate::types::*;
use std::sync::Arc;

pub struct Bucket {
    driver: Arc<dyn StorageDriver>,
    bucket: String,
}

impl Bucket {
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

pub struct FileBucket {
    storage: Bucket,
}

impl FileBucket {
    pub fn new(driver: Arc<dyn StorageDriver>, bucket: String) -> Self {
        Self {
            storage: Bucket::new(driver, bucket),
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

pub struct BackupBucket {
    storage: Bucket,
}

impl BackupBucket {
    pub fn new(driver: Arc<dyn StorageDriver>, config: &Config) -> Result<Self> {
        let bucket = config
            .backup_bucket
            .as_ref()
            .ok_or_else(|| Error::Config("backup_bucket not set".to_string()))?;

        Ok(Self {
            storage: Bucket::new(driver, bucket.clone()),
        })
    }

    pub async fn write_file(&self, path: &str, data: &[u8], public: bool) -> Result<()> {
        self.storage.write_file(path, data, public).await
    }
}
