//! This is how a single file is stored in the database.

use crate::types::*;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct FileRecord {
    pub id: u64,
    pub tree_id: u64,
    pub added_at: u64,
    pub added_by: u64,
    pub small_id: u64,
    pub large_id: u64,
}

impl FileRecord {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            tree_id: attributes.require_u64("tree_id")?,
            added_at: attributes.require_u64("added_at")?,
            added_by: attributes.require_u64("added_by")?,
            small_id: attributes.require_u64("small_id")?,
            large_id: attributes.require_u64("large_id")?,
        })
    }
}
