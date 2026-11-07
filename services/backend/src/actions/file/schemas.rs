use serde::Serialize;

use crate::domain::file::{file_url, File, FileDetails};
use crate::domain::tree_image::TreeImage;

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
    pub small_id: String,
    pub large_id: String,
    pub source_id: String,
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

impl From<TreeImage> for FileStatusResponse {
    fn from(file: TreeImage) -> Self {
        Self {
            ready: file.small_id > 0 && file.large_id > 0,
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
            small_id: file.id.to_string(),
            large_id: file.id.to_string(),
            source_id: file.id.to_string(),
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

    pub fn from_tree_image(image: &TreeImage, base_url: &str) -> Self {
        let url = |id: u64| {
            if id > 0 {
                Some(file_url(base_url, &id.to_string()))
            } else {
                None
            }
        };

        Self {
            id: image.id.to_string(),
            small_id: image.small_id.to_string(),
            large_id: image.large_id.to_string(),
            source_id: image.source_id.to_string(),
            added_at: image.added_at,
            added_by: image.added_by.to_string(),
            urls: FileUrls {
                original: url(image.source_id),
                large: url(image.large_id),
                small: url(image.small_id),
            },
            kind: "photo".to_string(),
            width: None,
            height: None,
        }
    }
}
