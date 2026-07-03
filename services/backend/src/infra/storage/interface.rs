use super::base::{CompletedPart, FileStorageInterface};
use super::local_storage::LocalFileStorage;
use super::s3_storage::S3FileStorage;
use crate::infra::config::Config;
use crate::infra::secrets::Secrets;
use crate::types::*;
use std::sync::Arc;

pub struct FileStorage {
    client: Arc<dyn FileStorageInterface>,
}

impl FileStorage {
    pub fn new(config: &Config, secrets: &Secrets) -> Result<Self> {
        if config.file_storage == "s3" {
            let client = Arc::new(S3FileStorage::new(config, secrets)?);
            return Ok(Self { client });
        }

        if config.file_storage == "local" {
            let client = Arc::new(LocalFileStorage::new(config));
            return Ok(Self { client });
        }

        Err(Error::Config(format!(
            "unsupported file storage type: {}",
            config.file_storage
        )))
    }

    pub async fn write_file(&self, id: u64, data: &[u8]) -> Result<()> {
        self.client.write_file(id, data).await
    }

    pub async fn read_file(&self, id: u64) -> Result<Vec<u8>> {
        self.client.read_file(id).await
    }

    pub async fn create_upload_url(&self, id: u64) -> Result<String> {
        self.client.create_upload_url(id).await
    }

    pub async fn exists(&self, key: &str) -> Result<bool> {
        self.client.exists(key).await
    }

    pub async fn start_multipart_upload(&self, key: &str) -> Result<String> {
        self.client.start_multipart_upload(key).await
    }

    pub async fn create_upload_part_url(
        &self,
        key: &str,
        upload_id: &str,
        part_number: i32,
    ) -> Result<String> {
        self.client
            .create_upload_part_url(key, upload_id, part_number)
            .await
    }

    pub async fn complete_multipart_upload(
        &self,
        key: &str,
        upload_id: &str,
        parts: Vec<CompletedPart>,
    ) -> Result<()> {
        self.client
            .complete_multipart_upload(key, upload_id, parts)
            .await
    }
}
