//! Link rows between trees and files.

use crate::infra::database::Attributes;
use crate::types::Result;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct TreeFile {
    pub id: u64,
    pub tree_id: u64,
    pub file_id: u64,
    pub sort_order: u64,
    pub added_at: u64,
    pub added_by: u64,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<u64>,
}

impl TreeFile {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            tree_id: attributes.require_u64("tree_id")?,
            file_id: attributes.require_u64("file_id")?,
            sort_order: attributes.get_u64("sort_order")?.unwrap_or(0),
            added_at: attributes.require_u64("added_at")?,
            added_by: attributes.require_u64("added_by")?,
            deleted_at: attributes.get_u64("deleted_at")?,
            deleted_by: attributes.get_u64("deleted_by")?,
        })
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}
