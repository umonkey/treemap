//! This module implements pluggable file storage drivers.

mod base;
mod interface;
mod local_storage;
mod s3_storage;

pub use interface::FileStorage;

// We need access to exact drivers when we move between storages.
pub use base::FileStorageInterface;
pub use local_storage::LocalFileStorage;
pub use s3_storage::S3FileStorage;
