use crate::infra::config::Config;
use crate::infra::storage::{LocalStorageDriver, S3StorageDriver, StorageDriver};
use crate::services::*;
use crate::types::*;
use log::{error, info, warn};
use std::sync::Arc;

pub struct MigrateService {
    local: Arc<dyn StorageDriver>,
    remote: Arc<dyn StorageDriver>,
    file_folder: String,
    files_bucket: String,
}

impl MigrateService {
    pub fn new(
        local: Arc<dyn StorageDriver>,
        remote: Arc<dyn StorageDriver>,
        config: Arc<Config>,
    ) -> Self {
        let files_bucket = config.files_bucket.clone().unwrap_or("files".to_string());
        Self {
            local,
            remote,
            file_folder: config.file_folder.clone(),
            files_bucket,
        }
    }

    pub async fn migrate_files(&self) -> Result<()> {
        let files = self.find_file_ids().expect("Error finding files.");

        let count = files.len();

        for id in files.into_iter() {
            let id_str = id.to_string();
            let data = self
                .local
                .read_file("", &id_str)
                .await
                .expect("Error reading files.");

            self.remote
                .write_file(&self.files_bucket, &id_str, &data, true)
                .await
                .expect("Error writing files.");
        }

        info!("Copied {} files.", count);

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

impl Injectable for MigrateService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        let config = ctx.config();
        let secrets = ctx.secrets();
        Ok(Self::new(
            Arc::new(LocalStorageDriver::new(&config)),
            Arc::new(S3StorageDriver::new(&config, &secrets)?),
            config,
        ))
    }
}
