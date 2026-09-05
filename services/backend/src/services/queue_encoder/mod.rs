use crate::services::queue_consumer::*;
use crate::types::*;
use log::error;

impl QueueCommand {
    pub fn decode(json: &str) -> Result<Option<Self>> {
        let value: serde_json::Value = serde_json::from_str(json).map_err(|e| {
            error!("Error parsing message: {e}, payload={json}");
            Error::Queue
        })?;

        let command = value["command"]
            .as_str()
            .ok_or("missing command")
            .map_err(|e| {
                error!("Error extracting message command: {e}, payload={json}");
                Error::Queue
            })?;

        let params = value["params"]
            .as_object()
            .ok_or("missing params")
            .map_err(|e| {
                error!("Error extracting message params: {e}, payload={json}");
                Error::Queue
            })?;

        match command {
            "ResizeImage" => {
                let id = params["id"].as_u64().ok_or("missing id").map_err(|e| {
                    error!("Error extracting image id: {e}, payload={json}");
                    Error::Queue
                })?;

                Ok(Some(QueueCommand::ResizeImage(ResizeImageMessage { id })))
            }

            "UpdateTreeAddress" => {
                let id = params["id"].as_u64().ok_or("missing id").map_err(|e| {
                    error!("Error extracting tree id: {e}, payload={json}");
                    Error::Queue
                })?;

                Ok(Some(QueueCommand::UpdateTreeAddress(
                    UpdateTreeAddressMessage { id },
                )))
            }

            "AddPhoto" => {
                let tree_id = params["tree_id"]
                    .as_u64()
                    .ok_or("missing tree_id")
                    .map_err(|e| {
                        error!("Error extracting tree id: {e}, payload={json}");
                        Error::Queue
                    })?;

                let file_id = params["file_id"]
                    .as_u64()
                    .ok_or("missing file_id")
                    .map_err(|e| {
                        error!("Error extracting file id: {e}, payload={json}");
                        Error::Queue
                    })?;

                Ok(Some(QueueCommand::AddPhoto(AddPhotoMessage {
                    tree_id,
                    file_id,
                })))
            }

            "UpdateUserpic" => {
                let user_id = params["user_id"]
                    .as_u64()
                    .ok_or("missing user_id")
                    .map_err(|e| {
                        error!("Error extracting user id: {e}, payload={json}");
                        Error::Queue
                    })?;

                let file_id = params["file_id"]
                    .as_u64()
                    .ok_or("missing file_id")
                    .map_err(|e| {
                        error!("Error extracting file id: {e}, payload={json}");
                        Error::Queue
                    })?;

                Ok(Some(QueueCommand::UpdateUserpic(UpdateUserpicMessage {
                    user_id,
                    file_id,
                })))
            }

            _ => {
                error!("Could not parse message: {json}");
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
#[allow(unreachable_patterns)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_resize_image() {
        let json = r#"{"command":"ResizeImage","params":{"id":12345}}"#;

        let command = QueueCommand::decode(json)
            .expect("Error parsing command.")
            .expect("No command found.");

        match command {
            QueueCommand::ResizeImage(message) => {
                assert_eq!(message.id, 12345);
            }

            _ => panic!("Expected ResizeImage, got {command:?}"),
        }
    }

    #[test]
    fn test_encode_resize_image() {
        let message = ResizeImageMessage { id: 12345 };

        assert_eq!(
            message.encode(),
            r#"{"command":"ResizeImage","params":{"id":12345}}"#
        );
    }
}
