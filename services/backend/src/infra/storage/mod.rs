//! This module implements pluggable file storage drivers.

mod aws_config;
mod base;
mod buckets;
mod interface;
mod local_storage;
mod s3_storage;

pub use base::{CompletedPart, StorageDriver};
pub use buckets::{BackupBucket, FileBucket};
pub use interface::create_driver;
pub use local_storage::LocalStorageDriver;
pub use s3_storage::S3StorageDriver;
