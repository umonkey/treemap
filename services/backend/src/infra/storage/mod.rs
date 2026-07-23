//! This module implements pluggable file storage drivers.

mod aws_config;
mod base;
mod interface;
mod local_storage;
mod s3_storage;

pub use base::{CompletedPart, StorageDriver};
pub use interface::{create_driver, BackupStorage, FileStorage};
pub use local_storage::LocalStorageDriver;
pub use s3_storage::S3StorageDriver;
