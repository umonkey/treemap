use serde::Serialize;

use crate::types::FileRecord;

#[derive(Debug, Serialize)]
pub struct FileUploadResponse {
    pub id: String,
}

impl FileUploadResponse {
    pub fn from_id(id: u64) -> Self {
        Self { id: id.to_string() }
    }

    pub fn from_file(file: &FileRecord) -> Self {
        Self {
            id: file.id.to_string(),
        }
    }
}
