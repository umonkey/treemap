use serde::Serialize;

use crate::domain::file::{file_url, File, FileDetails};

#[derive(Debug, Serialize)]
pub struct FileStatusResponse {
    pub ready: bool,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct FileUrls {
    pub original: Option<String>,
    pub large: Option<String>,
    pub small: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FileRead {
    pub id: String,
    pub added_at: u64,
    pub added_by: String,
    pub urls: FileUrls,
    pub kind: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl From<File> for FileStatusResponse {
    fn from(file: File) -> Self {
        Self {
            ready: file.status == crate::domain::file::FileStatus::Ready,
        }
    }
}

impl FileRead {
    pub fn from_file_details(details: &FileDetails, base_url: &str) -> Self {
        let file = &details.file;

        let url = |name: &str| {
            details
                .variant(name)
                .map(|variant| file_url(base_url, &variant.key))
        };

        Self {
            id: file.id.to_string(),
            added_at: file.created_at,
            added_by: file.created_by.to_string(),
            urls: FileUrls {
                original: url("original"),
                large: url("large"),
                small: url("small"),
            },
            kind: file.kind.as_str().to_string(),
            width: details.width(),
            height: details.height(),
        }
    }
}
