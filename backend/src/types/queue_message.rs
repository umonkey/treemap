//! This is how a single queue message is stored in the database.

use crate::types::*;
use rusqlite::types::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct QueueMessage {
    pub id: u64,
    pub added_at: u64,
    pub available_at: u64,
    pub payload: String,
    pub attempts: u64,
}

impl QueueMessage {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        attributes.deserialize::<Self>()
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
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
