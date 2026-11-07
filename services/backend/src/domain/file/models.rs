//! Universal file records and their derived variants.

use crate::infra::database::{Attributes, Value};
use crate::types::{Error, Result};
use serde::Serialize;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub enum FileKind {
    #[default]
    Photo,
    Video,
    Other,
}

impl FileKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Photo => "photo",
            Self::Video => "video",
            Self::Other => "other",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "video" => Self::Video,
            "other" => Self::Other,
            _ => Self::Photo,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub enum FileStatus {
    #[default]
    Pending,
    Processing,
    Ready,
    Failed,
}

impl FileStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Ready => "ready",
            Self::Failed => "failed",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "processing" => Self::Processing,
            "ready" => Self::Ready,
            "failed" => Self::Failed,
            _ => Self::Pending,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct File {
    pub id: u64,
    pub kind: FileKind,
    pub status: FileStatus,
    pub is_visible: bool,
    pub created_at: u64,
    pub created_by: u64,
    pub ready_at: Option<u64>,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<u64>,
}

impl File {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            kind: FileKind::from_str(&attributes.require_string("kind")?),
            status: FileStatus::from_str(&attributes.require_string("status")?),
            is_visible: attributes.get_bool("is_visible")?.unwrap_or(true),
            created_at: attributes.require_u64("created_at")?,
            created_by: attributes.require_u64("created_by")?,
            ready_at: attributes.get_u64("ready_at")?,
            deleted_at: attributes.get_u64("deleted_at")?,
            deleted_by: attributes.get_u64("deleted_by")?,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            ("kind".to_string(), Value::from(self.kind.as_str())),
            ("status".to_string(), Value::from(self.status.as_str())),
            (
                "is_visible".to_string(),
                Value::from(if self.is_visible { 1 } else { 0 }),
            ),
            (
                "created_at".to_string(),
                Value::from(self.created_at as i64),
            ),
            (
                "created_by".to_string(),
                Value::from(self.created_by as i64),
            ),
            ("ready_at".to_string(), Value::from(self.ready_at)),
            ("deleted_at".to_string(), Value::from(self.deleted_at)),
            ("deleted_by".to_string(), Value::from(self.deleted_by)),
        ])
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn is_listable(&self) -> bool {
        self.status == FileStatus::Ready && self.is_visible && !self.is_deleted()
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct FileVariant {
    pub file_id: u64,
    pub name: String,
    pub key: String,
    pub mime_type: String,
    pub size_bytes: u64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub created_at: u64,
}

impl FileVariant {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            file_id: attributes.require_u64("file_id")?,
            name: attributes.require_string("name")?,
            key: attributes.require_string("key")?,
            mime_type: attributes.require_string("mime_type")?,
            size_bytes: attributes.require_u64("size_bytes")?,
            width: attributes.get_u64("width")?.map(|value| value as u32),
            height: attributes.get_u64("height")?.map(|value| value as u32),
            created_at: attributes.require_u64("created_at")?,
        })
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct FileFallback {
    pub old_key: String,
    pub file_id: u64,
    pub variant: String,
}

impl FileFallback {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            old_key: attributes.require_string("old_key")?,
            file_id: attributes.require_u64("file_id")?,
            variant: attributes.require_string("variant")?,
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct FileDetails {
    pub file: File,
    pub variants: Vec<FileVariant>,
}

impl FileDetails {
    pub fn variant(&self, name: &str) -> Option<&FileVariant> {
        self.variants.iter().find(|variant| variant.name == name)
    }

    pub fn original(&self) -> Option<&FileVariant> {
        self.variant("original")
    }

    pub fn small(&self) -> Option<&FileVariant> {
        self.variant("small")
    }

    pub fn large(&self) -> Option<&FileVariant> {
        self.variant("large")
    }

    pub fn width(&self) -> Option<u32> {
        self.small()
            .or_else(|| self.large())
            .or_else(|| self.original())
            .and_then(|variant| variant.width)
    }

    pub fn height(&self) -> Option<u32> {
        self.small()
            .or_else(|| self.large())
            .or_else(|| self.original())
            .and_then(|variant| variant.height)
    }

    pub fn is_listable(&self) -> bool {
        self.file.is_listable()
    }
}

/// Build a public URL from the configured base URL and a storage key.
pub fn file_url(base_url: &str, key: &str) -> String {
    if key.is_empty() {
        return base_url.to_string();
    }

    if base_url.is_empty() {
        return key.to_string();
    }

    if base_url.ends_with('/') {
        format!("{base_url}{key}")
    } else {
        format!("{base_url}/{key}")
    }
}

/// Parse a list of string file ids from an API payload.
pub fn parse_file_ids(files: &[String]) -> Result<Vec<u64>> {
    files
        .iter()
        .map(|file| {
            file.parse::<u64>()
                .map_err(|_| Error::BadRequestMessage(format!("Invalid file id: {file}")))
        })
        .collect()
}
