use log::{error, info, warn};
use std::sync::Arc;

use crate::services::{FileStorageInterface, LocalFileStorage, S3FileStorage};
use crate::types::Result;
use crate::utils::get_file_folder;

fn find_file_ids() -> Result<Vec<u64>> {
    let mut res: Vec<u64> = Vec::new();
    let pattern = format!("{}/[0-9]*", get_file_folder());

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
                        warn!("Error parsing file name: {}", e);
                    }
                }
            }

            Err(e) => {
                error!("Error listing files: {}", e);
            }
        }
    }

    Ok(res)
}

pub async fn upload_files_command() {
    let local: Arc<dyn FileStorageInterface> = Arc::new(LocalFileStorage::new());

    let remote: Arc<dyn FileStorageInterface> = Arc::new(
        S3FileStorage::new()
            .await
            .expect("Error creating S3 client"),
    );

    let files = find_file_ids().expect("Error finding files.");

    let count = files.len();

    for id in files.into_iter() {
        let data = local.read_file(id).await.expect("Error reading files.");
        remote
            .write_file(id, &data)
            .await
            .expect("Error writing files.");
    }

    info!("Copied {} files.", count);
}
