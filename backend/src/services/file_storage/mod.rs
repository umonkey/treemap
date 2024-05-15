/**
 * This module defines file storage backends.
 */
mod local_storage;
mod s3_storage;
mod r#trait;

pub use self::local_storage::*;
pub use self::r#trait::*;
pub use self::s3_storage::*;

use crate::types::Result;
use std::sync::Arc;

pub async fn get_file_storage() -> Result<Arc<dyn FileStorageInterface>> {
    if let Ok(value) = S3FileStorage::new().await {
        return Ok(Arc::new(value));
    }

    Ok(Arc::new(LocalFileStorage::new()))
}
