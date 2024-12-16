use crate::types::FileRecord;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PublicFileInfo {
    pub id: String,
    pub small_id: String,
    pub large_id: String,
    pub added_at: u64,
    pub added_by: String,
}

impl PublicFileInfo {
    pub fn from_file(file: &FileRecord) -> PublicFileInfo {
        PublicFileInfo {
            id: file.id.to_string(),
            small_id: file.small_id.to_string(),
            large_id: file.large_id.to_string(),
            added_at: file.added_at,
            added_by: file.added_by.to_string(),
        }
    }
}
