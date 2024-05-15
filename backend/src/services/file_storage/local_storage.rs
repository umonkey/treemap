use async_trait::async_trait;
use log::{debug, error, info};
use tokio::fs;

use crate::services::FileStorageInterface;
use crate::types::{Error, Result};
use crate::utils::get_file_folder;

pub struct LocalFileStorage {
    folder: String,
}

impl LocalFileStorage {
    pub fn new() -> Self {
        Self {
            folder: get_file_folder(),
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
                error!("Error creating folder: {:?}", e);
                return Err(Error::FileUpload);
            }
        };

        match fs::write(file_path, data).await {
            Ok(()) => {
                info!("File {} written, {} bytes.", id, data.len());
                Ok(())
            }

            Err(e) => {
                error!("Error writing file: {:?}", e);
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
                debug!("File {} not found.", id);
                Err(Error::FileNotFound)
            }

            Err(e) => {
                error!("Error reading file {}: {:?}", id, e);
                Err(Error::FileDownload)
            }
        }
    }
}
