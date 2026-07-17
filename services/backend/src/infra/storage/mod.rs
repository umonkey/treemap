//! This module implements pluggable file storage drivers.

mod aws_config;
mod base;
mod interface;
mod local_storage;
mod s3_storage;

pub use interface::{create_driver, FileStorage};

// We need access to exact drivers when we move between storages.
pub use base::StorageDriver;
pub use local_storage::LocalStorageDriver;
pub use s3_storage::S3StorageDriver;
