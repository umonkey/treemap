use serde::Serialize;

use crate::types::FileRecord;

#[derive(Debug, Serialize)]
pub struct FileStatusResponse {
    pub ready: bool,
}

impl FileStatusResponse {
    pub fn from_file(file: &FileRecord) -> Self {
        Self {
            ready: file.small_id > 0 && file.large_id > 0,
        }
    }
}
