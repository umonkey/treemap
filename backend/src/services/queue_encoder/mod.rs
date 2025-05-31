use crate::types::*;
use log::{error, trace};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;

/// This is used for simpler parsing.
#[derive(Debug, Deserialize)]
pub struct MessageHeader {
    command: String,
    params: serde_json::Value,
}

impl QueueCommand {
    pub fn decode(json: &str) -> Result<Option<Self>> {
        trace!("Decoding a queue command from JSON: {json}");

        let header: MessageHeader = serde_json::from_str(json).map_err(|e| {
            error!("Error parsing message header: {}, payload={}", e, json);
            Error::Queue
        })?;

        trace!("Parsed header: {:?}", header);

        match header.command.as_str() {
            "ResizeImage" => {
                let msg = Self::decode_params::<ResizeImageMessage>(header.params)?;
                Ok(Some(QueueCommand::ResizeImage(msg)))
            }

            "UpdateTreeAddress" => {
                let msg = Self::decode_params::<UpdateTreeAddressMessage>(header.params)?;
                Ok(Some(QueueCommand::UpdateTreeAddress(msg)))
            }

            "AddPhoto" => {
                let msg = Self::decode_params::<AddPhotoMessage>(header.params)?;
                Ok(Some(QueueCommand::AddPhoto(msg)))
            }

            "UpdateUserpic" => {
                let msg = Self::decode_params::<UpdateUserpicMessage>(header.params)?;
                Ok(Some(QueueCommand::UpdateUserpic(msg)))
            }

            "AddStory" => {
                let msg = Self::decode_params::<AddStoryMessage>(header.params)?;
                Ok(Some(QueueCommand::AddStory(msg)))
            }

            _ => {
                error!("Could not parse message: {}", json);
                Ok(None)
            }
        }
    }

    fn decode_params<T>(value: Value) -> Result<T>
    where
        T: DeserializeOwned
    {
        let msg: T = serde_json::from_value(value.clone())
            .map_err(|e| {
                error!("Error parsing AddStory message: {}, payload={}", e, value);
                Error::Queue
            })?;

        Ok(msg)
    }
}

#[cfg(test)]
#[allow(unreachable_patterns)]
mod tests {
    use super::*;

    fn setup() {
        env_logger::try_init().expect("Failed to initialize logger");
    }

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

    #[test]
    fn test_decode_add_story() {
        setup();

        let json = AddStoryMessage {
            user_id: 1,
            tree_id: 2,
            file_id: 3,
            added_at: 1234567890,
            text: Some("Test story".to_string()),
        }.encode();

        let command = QueueCommand::decode(&json)
            .expect("Error parsing command.")
            .expect("No command found.");

        let msg = match command {
            QueueCommand::AddStory(message) => message,
            _ => panic!("Expected AddStory, got {:?}", command),
        };

        assert_eq!(msg.tree_id, 2);
    }
}
