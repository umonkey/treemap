use serde::{Deserialize, Serialize};
use serde_json::json;

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

#[derive(Debug, Serialize)]
pub enum QueueCommand {
    ResizeImage(ResizeImageMessage),
    UpdateTreeAddress(UpdateTreeAddressMessage),
    AddPhoto(AddPhotoMessage),
    UpdateUserpic(UpdateUserpicMessage),
    AddStory(AddStoryMessage),
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

impl AddStoryMessage {
    pub fn encode(&self) -> String {
        json!({
            "command": "AddStory",
            "params": {
                "user_id": self.user_id,
                "tree_id": self.tree_id,
                "file_id": self.file_id,
                "added_at": self.added_at,
            },
        })
        .to_string()
    }
}
