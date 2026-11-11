use serde::{Deserialize, Serialize};
use serde_json::json;

// Process a universal file upload into its derived variants.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessFileMessage {
    pub id: u64,
}

// Request tree street address using Nominatim.
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTreeAddressMessage {
    pub id: u64,
}

// Add a photo to a tree.
#[derive(Debug, Deserialize, Serialize)]
pub struct AddPhotoMessage {
    pub tree_id: u64,
    pub file_id: u64,
}

// Update a profile picture.
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserpicMessage {
    pub user_id: u64,
    pub file_id: u64,
}

// Recompute the pre-rendered track geometry and bounds for a panorama.
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePanoramaStatsMessage {
    pub id: u64,
}

// Copy the AWS Batch job log into the panorama folder.
#[derive(Debug, Deserialize, Serialize)]
pub struct DownloadPanoramaLogsMessage {
    pub id: u64,
    pub log_stream: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum QueueCommand {
    ProcessFile(ProcessFileMessage),
    UpdateTreeAddress(UpdateTreeAddressMessage),
    AddPhoto(AddPhotoMessage),
    UpdateUserpic(UpdateUserpicMessage),
    UpdatePanoramaStats(UpdatePanoramaStatsMessage),
    DownloadPanoramaLogs(DownloadPanoramaLogsMessage),
}

impl ProcessFileMessage {
    pub fn encode(&self) -> String {
        json!({
            "command": "ProcessFile",
            "params": {
                "id": self.id,
            },
        })
        .to_string()
    }
}

impl UpdateTreeAddressMessage {
    pub fn encode(&self) -> String {
        json!({
            "command": "UpdateTreeAddress",
            "params": {
                "id": self.id,
            },
        })
        .to_string()
    }
}

impl AddPhotoMessage {
    pub fn encode(&self) -> String {
        json!({
            "command": "AddPhoto",
            "params": {
                "tree_id": self.tree_id,
                "file_id": self.file_id,
            },
        })
        .to_string()
    }
}

impl UpdateUserpicMessage {
    pub fn encode(&self) -> String {
        json!({
            "command": "UpdateUserpic",
            "params": {
                "user_id": self.user_id,
                "file_id": self.file_id,
            },
        })
        .to_string()
    }
}

impl UpdatePanoramaStatsMessage {
    pub fn encode(&self) -> String {
        json!({
            "command": "UpdatePanoramaStats",
            "params": {
                "id": self.id,
            },
        })
        .to_string()
    }
}

impl DownloadPanoramaLogsMessage {
    pub fn encode(&self) -> String {
        json!({
            "command": "DownloadPanoramaLogs",
            "params": {
                "id": self.id,
                "log_stream": self.log_stream,
            },
        })
        .to_string()
    }
}
