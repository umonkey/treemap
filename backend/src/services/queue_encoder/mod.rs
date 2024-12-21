use crate::types::*;
use log::error;

impl QueueCommand {
    pub fn decode(json: &str) -> Result<Option<Self>> {
        let value: serde_json::Value = serde_json::from_str(json).map_err(|e| {
            error!("Error parsing message: {}, payload={}", e, json);
            Error::Queue
        })?;

        let command = value["command"]
            .as_str()
            .ok_or("missing command")
            .map_err(|e| {
                error!("Error extracting message command: {}, payload={}", e, json);
                Error::Queue
            })?;

        let params = value["params"]
            .as_object()
            .ok_or("missing params")
            .map_err(|e| {
                error!("Error extracting message params: {}, payload={}", e, json);
                Error::Queue
            })?;

        match command {
            "ResizeImage" => {
                let id = params["id"].as_u64().ok_or("missing id").map_err(|e| {
                    error!("Error extracting image id: {}, payload={}", e, json);
                    Error::Queue
                })?;

                Ok(Some(QueueCommand::ResizeImage(ResizeImageMessage { id })))
            }

            "UpdateTreeAddress" => {
                let id = params["id"].as_u64().ok_or("missing id").map_err(|e| {
                    error!("Error extracting tree id: {}, payload={}", e, json);
                    Error::Queue
                })?;

                Ok(Some(QueueCommand::UpdateTreeAddress(
                    UpdateTreeAddressMessage { id },
                )))
            }

            _ => {
                error!("Could not parse message: {}", json);
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

            _ => panic!("Expected ResizeImage, got {:?}", command),
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
