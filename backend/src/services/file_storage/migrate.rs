use log::info;
use std::sync::Arc;

use crate::services::{FileStorageInterface, LocalFileStorage, S3FileStorage};
use crate::types::Result;

pub async fn migrate_local_to_remote() -> Result<()> {
    let local: Arc<dyn FileStorageInterface> = Arc::new(LocalFileStorage::new());
    let remote: Arc<dyn FileStorageInterface> = Arc::new(S3FileStorage::new().await?);

    let files = local.find_files().await?;
    let count = files.len();

    for id in files.into_iter() {
        let data = local.read_file(id).await?;
        remote.write_file(id, &data).await?;
    }

    info!("Copied {} files.", count);

    Ok(())
}
