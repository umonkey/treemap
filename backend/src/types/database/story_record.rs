use crate::types::{Attributes, Result};
use rusqlite::types::Value;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
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
        Ok(Self {
            id: attributes.require_u64("id")?,
            user_id: attributes.require_u64("user_id")?,
            tree_id: attributes.require_u64("tree_id")?,
            file_id: attributes.require_u64("file_id")?,
            added_at: attributes.require_u64("added_at")?,
            text: attributes.get_string("text")?,
        })
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
