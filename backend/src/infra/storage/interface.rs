use super::base::FileStorageInterface;
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
}
