//! Create the correct file storage driver based on the config.

use super::base::StorageDriver;
use super::local_storage::LocalStorageDriver;
use super::s3_storage::S3StorageDriver;
use crate::infra::config::Config;
use crate::infra::secrets::Secrets;
use crate::types::*;
use std::sync::Arc;

pub fn create_driver(config: &Config, secrets: &Secrets) -> Result<Arc<dyn StorageDriver>> {
    if config.file_storage == "s3" {
        return Ok(Arc::new(S3StorageDriver::new(config, secrets)?));
    }

    if config.file_storage == "local" {
        return Ok(Arc::new(LocalStorageDriver::new(config)));
    }

    Err(Error::Config(format!(
        "unsupported file storage type: {}",
        config.file_storage
    )))
}
