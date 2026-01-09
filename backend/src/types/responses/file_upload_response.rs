use serde::Serialize;

use crate::domain::file::File;

#[derive(Debug, Serialize)]
pub struct FileUploadResponse {
    pub id: String,
}

impl FileUploadResponse {
    pub fn from_id(id: u64) -> Self {
        Self { id: id.to_string() }
    }

    pub fn from_file(file: &File) -> Self {
        Self {
            id: file.id.to_string(),
        }
    }
}
