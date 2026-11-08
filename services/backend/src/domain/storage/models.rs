//! Models for the object storage inventory.

/// One object observed in a storage bucket.
#[derive(Clone, Debug)]
pub struct StorageObject {
    pub bucket: String,
    pub key: String,
    pub size_bytes: u64,
    pub etag: Option<String>,
    pub last_modified_at: u64,
}
