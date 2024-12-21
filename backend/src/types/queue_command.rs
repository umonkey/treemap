use serde::{Deserialize, Serialize};
use serde_json::json;

/**
 * Prepare images from an uploaded file.
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct ResizeImageMessage {
    pub id: u64,
}

// Request tree street address using Nominatim.
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTreeAddressMessage {
    pub id: u64,
}

#[derive(Debug, Serialize)]
pub enum QueueCommand {
    ResizeImage(ResizeImageMessage),
    UpdateTreeAddress(UpdateTreeAddressMessage),
}

impl ResizeImageMessage {
    pub fn encode(&self) -> String {
        json!({
            "command": "ResizeImage",
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
