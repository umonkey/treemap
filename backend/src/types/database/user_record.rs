use crate::types::{Attributes, Result};
use rusqlite::types::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UserRecord {
    pub id: u64,
    pub email: String,
    pub name: String,
    pub picture: String,
    pub trees_count: i64,
    pub comments_count: i64,
    pub updates_count: i64,
    pub files_count: i64,
}

impl UserRecord {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        attributes.deserialize::<Self>()
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            ("email".to_string(), Value::from(self.email.clone())),
            ("name".to_string(), Value::from(self.name.clone())),
            ("picture".to_string(), Value::from(self.picture.clone())),
            ("trees_count".to_string(), Value::from(self.trees_count)),
            (
                "comments_count".to_string(),
                Value::from(self.comments_count),
            ),
            ("updates_count".to_string(), Value::from(self.updates_count)),
            ("files_count".to_string(), Value::from(self.files_count)),
        ])
    }
}
