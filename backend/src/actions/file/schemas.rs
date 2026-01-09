use serde::Serialize;

use crate::domain::file::File;

#[derive(Debug, Serialize)]
pub struct FileStatusResponse {
    pub ready: bool,
}

#[derive(Debug, Serialize)]
pub struct FileRead {
    pub id: String,
    pub small_id: String,
    pub large_id: String,
    pub added_at: u64,
    pub added_by: String,
}

impl From<File> for FileStatusResponse {
    fn from(file: File) -> Self {
        Self {
            ready: file.small_id > 0 && file.large_id > 0,
        }
    }
}

impl From<File> for FileRead {
    fn from(file: File) -> Self {
        Self {
            id: file.id.to_string(),
            small_id: file.small_id.to_string(),
            large_id: file.large_id.to_string(),
            added_at: file.added_at,
            added_by: file.added_by.to_string(),
        }
    }
}

impl From<&File> for FileRead {
    fn from(file: &File) -> Self {
        Self::from(file.clone())
    }
}
