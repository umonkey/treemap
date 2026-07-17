use super::base::StorageDriver;
use crate::infra::config::Config;
use crate::types::*;
use async_trait::async_trait;
use log::{debug, error, info};
use tokio::fs;

pub struct LocalStorageDriver {
    folder: String,
}

impl LocalStorageDriver {
    pub fn new(config: &Config) -> Self {
        Self {
            folder: config.file_folder.clone(),
        }
    }
}

#[async_trait]
impl StorageDriver for LocalStorageDriver {
    async fn write_file(&self, bucket: &str, path: &str, data: &[u8], _public: bool) -> Result<()> {
        let bucket_folder = format!("{}/{}", self.folder, bucket);
        let file_path = format!("{}/{}", bucket_folder, path);

        match fs::create_dir_all(&bucket_folder).await {
            Ok(()) => (),

            Err(e) => {
                error!("Error creating folder {}: {:?}", bucket_folder, e);
                return Err(Error::FileUpload);
            }
        };

        match fs::write(file_path, data).await {
            Ok(()) => {
                info!("File {}/{} written, {} bytes.", bucket, path, data.len());
                Ok(())
            }

            Err(e) => {
                error!("Error writing file: {:?}", e);
                Err(Error::FileUpload)
            }
        }
    }

    async fn read_file(&self, bucket: &str, path: &str) -> Result<Vec<u8>> {
        let file_path = format!("{}/{}/{}", self.folder, bucket, path);

        match fs::read(file_path).await {
            Ok(value) => {
                debug!("File {}/{} read, {} bytes.", bucket, path, value.len());
                Ok(value)
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                debug!("File {}/{} not found.", bucket, path);
                Err(Error::FileNotFound)
            }

            Err(e) => {
                error!("Error reading file {}/{}: {:?}", bucket, path, e);
                Err(Error::FileDownload)
            }
        }
    }

    async fn create_upload_url(&self, _bucket: &str, _path: &str) -> Result<String> {
        Err(Error::FileUpload)
    }
}
