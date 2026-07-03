use super::base::{CompletedPart, FileStorageInterface};
use crate::infra::config::Config;
use crate::types::*;
use async_trait::async_trait;
use log::{debug, error, info};
use tokio::fs;

pub struct LocalFileStorage {
    folder: String,
}

impl LocalFileStorage {
    pub fn new(config: &Config) -> Self {
        Self {
            folder: config.file_folder.clone(),
        }
    }
}

#[async_trait]
impl FileStorageInterface for LocalFileStorage {
    async fn write_file(&self, id: u64, data: &[u8]) -> Result<()> {
        let file_path = format!("{}/{}", self.folder, id);

        match fs::create_dir_all(&self.folder).await {
            Ok(()) => (),

            Err(e) => {
                error!("Error creating folder: {e:?}");
                return Err(Error::FileUpload);
            }
        };

        match fs::write(file_path, data).await {
            Ok(()) => {
                info!("File {} written, {} bytes.", id, data.len());
                Ok(())
            }

            Err(e) => {
                error!("Error writing file: {e:?}");
                Err(Error::FileUpload)
            }
        }
    }

    async fn read_file(&self, id: u64) -> Result<Vec<u8>> {
        let file_path = format!("{}/{}", self.folder, id);

        match fs::read(file_path).await {
            Ok(value) => {
                debug!("File {} read, {} bytes.", id, value.len());
                Ok(value)
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                debug!("File {id} not found.");
                Err(Error::FileNotFound)
            }

            Err(e) => {
                error!("Error reading file {id}: {e:?}");
                Err(Error::FileDownload)
            }
        }
    }

    async fn create_upload_url(&self, _id: u64) -> Result<String> {
        Err(Error::FileUpload)
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        let file_path = format!("{}/{}", self.folder, key);
        Ok(fs::metadata(file_path).await.is_ok())
    }

    async fn start_multipart_upload(&self, _key: &str) -> Result<String> {
        Err(Error::FileUpload)
    }

    async fn create_upload_part_url(
        &self,
        _key: &str,
        _upload_id: &str,
        _part_number: i32,
    ) -> Result<String> {
        Err(Error::FileUpload)
    }

    async fn complete_multipart_upload(
        &self,
        _key: &str,
        _upload_id: &str,
        _parts: Vec<CompletedPart>,
    ) -> Result<()> {
        Err(Error::FileUpload)
    }
}
