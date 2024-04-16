/**
 * This module contains types used for passing queue messages.
 */
use serde::{Deserialize, Serialize};
use serde_json::json;

/**
 * Prepare images from an uploaded file.
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct ResizeImageMessage {
    pub id: u64,
}

#[derive(Debug, Serialize)]
pub enum QueueCommand {
    ResizeImage(ResizeImageMessage),
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
