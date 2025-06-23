use crate::types::{Error, Result};
use log::trace;
use serde::{Deserialize, Serialize};

// Prepare images from an uploaded file.
#[derive(Debug, Deserialize, Serialize)]
pub struct ResizeImageMessage {
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

// Attempt to send a photo as a story.
#[derive(Debug, Deserialize, Serialize)]
pub struct AddStoryMessage {
    pub user_id: u64,
    pub tree_id: u64,
    pub file_id: u64,
    pub added_at: u64,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command", content = "params")]
pub enum QueueCommand {
    ResizeImage(ResizeImageMessage),
    UpdateTreeAddress(UpdateTreeAddressMessage),
    AddPhoto(AddPhotoMessage),
    UpdateUserpic(UpdateUserpicMessage),
    AddStory(AddStoryMessage),
}

impl QueueCommand {
    pub fn encode(&self) -> Result<String> {
        trace!("Serializing QueueCommand {:?}", self);

        let json = serde_json::to_string(self).map_err(|e| {
            log::error!("Error serializing QueueCommand: {}", e);
            Error::Queue
        })?;

        Ok(json.to_string())
    }

    pub fn decode(json: &str) -> Result<QueueCommand> {
        trace!("Deserializing QueueCommand from JSON: {}", json);

        let command: QueueCommand = serde_json::from_str(json).map_err(|e| {
            log::error!("Error deserializing QueueCommand: {}", e);
            Error::Queue
        })?;

        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        _ = env_logger::try_init();
    }

    #[test]
    fn test_decode() {
        setup();

        let command = QueueCommand::decode(r#"{"command":"ResizeImage","params":{"id":12345}}"#)
            .expect("Error parsing command.");

        if let QueueCommand::ResizeImage(msg) = command {
            assert_eq!(msg.id, 12345);
        } else {
            panic!("Expected ResizeImage command.");
        }
    }

    #[test]
    fn test_encode() {
        setup();

        let json = QueueCommand::ResizeImage(ResizeImageMessage { id: 12345 })
            .encode()
            .expect("Error encoding command.");

        assert_eq!(json, r#"{"command":"ResizeImage","params":{"id":12345}}"#);
    }
}
