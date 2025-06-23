//! This record represents a single file upload.
//! File uploads are not connected to any tree, just orphan files
//! that are then used to create tree photos by submitting ids.

use crate::types::*;
use rusqlite::types::Value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UploadRecord {
    pub id: u64,
    pub added_at: u64,
    pub added_by: u64,
    pub size: u64,
}

impl UploadRecord {
    #[allow(unused)]
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        attributes.deserialize::<Self>()
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            ("added_at".to_string(), Value::from(self.added_at as i64)),
            ("added_by".to_string(), Value::from(self.added_by as i64)),
            ("size".to_string(), Value::from(self.size as i64)),
        ])
    }
}
