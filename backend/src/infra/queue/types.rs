use crate::infra::database::{Attributes, Value};
use crate::types::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct QueueMessage {
    pub id: String,
    pub added_at: u64,
    pub available_at: u64,
    pub payload: String,
    pub attempts: u64,
}

impl QueueMessage {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?.to_string(),
            added_at: attributes.require_u64("added_at")?,
            available_at: attributes.require_u64("available_at")?,
            payload: attributes.require_string("payload")?,
            attempts: attributes.require_u64("attempts")?,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            (
                "id".to_string(),
                Value::from(self.id.parse::<i64>().unwrap_or_default()),
            ),
            ("added_at".to_string(), Value::from(self.added_at as i64)),
            (
                "available_at".to_string(),
                Value::from(self.available_at as i64),
            ),
            ("payload".to_string(), Value::from(self.payload.clone())),
            ("attempts".to_string(), Value::from(self.attempts as i64)),
        ])
    }
}
