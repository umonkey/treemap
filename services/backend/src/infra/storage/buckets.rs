//! Defines wrappers for the different storage buckets.

use super::base::{CompletedPart, StorageDriver};
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

    pub fn name(&self) -> &str {
        &self.bucket
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

    pub async fn create_read_url(&self, path: &str) -> Result<String> {
        self.driver.create_read_url(&self.bucket, path).await
    }

    pub async fn exists(&self, path: &str) -> Result<bool> {
        self.driver.exists(&self.bucket, path).await
    }

    pub async fn start_multipart_upload(&self, path: &str) -> Result<String> {
        self.driver.start_multipart_upload(&self.bucket, path).await
    }

    pub async fn create_upload_part_url(
        &self,
        path: &str,
        upload_id: &str,
        part_number: i32,
    ) -> Result<String> {
        self.driver
            .create_upload_part_url(&self.bucket, path, upload_id, part_number)
            .await
    }

    pub async fn complete_multipart_upload(
        &self,
        path: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<()> {
        self.driver
            .complete_multipart_upload(&self.bucket, path, upload_id, parts)
            .await
    }
}

pub struct FileBucket {
    storage: Bucket,
}

#[allow(dead_code)]
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

    pub async fn exists(&self, key: &str) -> Result<bool> {
        self.storage.exists(key).await
    }

    pub async fn start_multipart_upload(&self, key: &str) -> Result<String> {
        self.storage.start_multipart_upload(key).await
    }

    pub async fn create_upload_part_url(
        &self,
        key: &str,
        upload_id: &str,
        part_number: i32,
    ) -> Result<String> {
        self.storage
            .create_upload_part_url(key, upload_id, part_number)
            .await
    }

    pub async fn complete_multipart_upload(
        &self,
        key: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<()> {
        self.storage
            .complete_multipart_upload(key, upload_id, parts)
            .await
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

pub struct PanoramaSourceBucket {
    storage: Bucket,
}

#[allow(dead_code)]
impl PanoramaSourceBucket {
    pub fn new(driver: Arc<dyn StorageDriver>, config: &Config) -> Result<Self> {
        let bucket = config
            .panoramas_sources_bucket
            .clone()
            .unwrap_or_else(|| "treemap-panoramas-source".to_string());

        Ok(Self {
            storage: Bucket::new(driver, bucket),
        })
    }

    pub fn name(&self) -> &str {
        self.storage.name()
    }

    pub async fn write_file(&self, path: &str, data: &[u8], public: bool) -> Result<()> {
        self.storage.write_file(path, data, public).await
    }

    pub async fn read_file(&self, path: &str) -> Result<Vec<u8>> {
        self.storage.read_file(path).await
    }

    pub async fn create_upload_url(&self, path: &str) -> Result<String> {
        self.storage.create_upload_url(path).await
    }

    pub async fn create_read_url(&self, path: &str) -> Result<String> {
        self.storage.create_read_url(path).await
    }

    pub async fn exists(&self, key: &str) -> Result<bool> {
        self.storage.exists(key).await
    }

    pub async fn start_multipart_upload(&self, key: &str) -> Result<String> {
        self.storage.start_multipart_upload(key).await
    }

    pub async fn create_upload_part_url(
        &self,
        key: &str,
        upload_id: &str,
        part_number: i32,
    ) -> Result<String> {
        self.storage
            .create_upload_part_url(key, upload_id, part_number)
            .await
    }

    pub async fn complete_multipart_upload(
        &self,
        key: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<()> {
        self.storage
            .complete_multipart_upload(key, upload_id, parts)
            .await
    }
}

pub struct PanoramaBucket {
    storage: Bucket,
}

#[allow(dead_code)]
impl PanoramaBucket {
    pub fn new(driver: Arc<dyn StorageDriver>, config: &Config) -> Result<Self> {
        let bucket = config
            .panoramas_bucket
            .clone()
            .unwrap_or_else(|| "treemap-panoramas".to_string());

        Ok(Self {
            storage: Bucket::new(driver, bucket),
        })
    }

    pub fn name(&self) -> &str {
        self.storage.name()
    }

    pub async fn write_file(&self, path: &str, data: &[u8], public: bool) -> Result<()> {
        self.storage.write_file(path, data, public).await
    }

    pub async fn read_file(&self, path: &str) -> Result<Vec<u8>> {
        self.storage.read_file(path).await
    }

    pub async fn create_upload_url(&self, path: &str) -> Result<String> {
        self.storage.create_upload_url(path).await
    }

    pub async fn create_read_url(&self, path: &str) -> Result<String> {
        self.storage.create_read_url(path).await
    }

    pub async fn exists(&self, key: &str) -> Result<bool> {
        self.storage.exists(key).await
    }

    pub async fn start_multipart_upload(&self, key: &str) -> Result<String> {
        self.storage.start_multipart_upload(key).await
    }

    pub async fn create_upload_part_url(
        &self,
        key: &str,
        upload_id: &str,
        part_number: i32,
    ) -> Result<String> {
        self.storage
            .create_upload_part_url(key, upload_id, part_number)
            .await
    }

    pub async fn complete_multipart_upload(
        &self,
        key: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<()> {
        self.storage
            .complete_multipart_upload(key, upload_id, parts)
            .await
    }
}
