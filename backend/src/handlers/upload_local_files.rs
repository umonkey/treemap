use crate::infra::config::Config;
use crate::infra::storage::{FileStorageInterface, LocalFileStorage, S3FileStorage};
use crate::services::*;
use crate::types::*;
use log::{error, info, warn};
use std::sync::Arc;

pub struct UploadLocalFiles {
    local: Arc<dyn FileStorageInterface>,
    remote: Arc<dyn FileStorageInterface>,
    file_folder: String,
}

impl UploadLocalFiles {
    pub fn new(
        local: Arc<dyn FileStorageInterface>,
        remote: Arc<dyn FileStorageInterface>,
        config: Arc<Config>,
    ) -> Self {
        Self {
            local,
            remote,
            file_folder: config.file_folder.clone(),
        }
    }

    pub async fn handle(&self) -> Result<()> {
        let files = self.find_file_ids().expect("Error finding files.");

        let count = files.len();

        for id in files.into_iter() {
            let data = self
                .local
                .read_file(id)
                .await
                .expect("Error reading files.");

            self.remote
                .write_file(id, &data)
                .await
                .expect("Error writing files.");
        }

        info!("Copied {count} files.");

        Ok(())
    }

    fn find_file_ids(&self) -> Result<Vec<u64>> {
        let mut res: Vec<u64> = Vec::new();
        let pattern = format!("{}/[0-9]*", self.file_folder);

        for entry in glob::glob(pattern.as_str()).expect("Error listing files.") {
            match entry {
                Ok(path) => {
                    let name = path
                        .file_name()
                        .expect("Error extracting file name.")
                        .to_str()
                        .expect("Error converting file name to string.");

                    match name.parse::<u64>() {
                        Ok(id) => res.push(id),

                        Err(e) => {
                            warn!("Error parsing file name: {e}");
                        }
                    }
                }

                Err(e) => {
                    error!("Error listing files: {e}");
                }
            }
        }

        Ok(res)
    }
}

impl Locatable for UploadLocalFiles {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self::new(
            locator.get::<LocalFileStorage>()?,
            locator.get::<S3FileStorage>()?,
            locator.get::<Config>()?,
        ))
    }
}
