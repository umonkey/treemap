use crate::types::{Attributes, Result};
use rusqlite::types::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct StoryRecord {
    pub id: u64,
    pub user_id: u64,
    pub tree_id: u64,
    pub file_id: u64,
    pub added_at: u64,
    pub text: Option<String>,
}

impl StoryRecord {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        attributes.deserialize::<Self>()
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            ("user_id".to_string(), Value::from(self.user_id as i64)),
            ("tree_id".to_string(), Value::from(self.tree_id as i64)),
            ("file_id".to_string(), Value::from(self.file_id as i64)),
            ("added_at".to_string(), Value::from(self.added_at as i64)),
            (
                "text".to_string(),
                Value::from(self.text.clone().unwrap_or_default()),
            ),
        ])
    }
}
