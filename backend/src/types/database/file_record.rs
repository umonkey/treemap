//! This is how a single file is stored in the database.

use crate::infra::database::{Attributes, Value};
use crate::types::*;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct FileRecord {
    pub id: u64,
    pub tree_id: u64,
    pub added_at: u64,
    pub added_by: u64,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<u64>,
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
            deleted_at: attributes.get_u64("deleted_at")?,
            deleted_by: attributes.get_u64("deleted_by")?,
            small_id: attributes.require_u64("small_id")?,
            large_id: attributes.require_u64("large_id")?,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            ("tree_id".to_string(), Value::from(self.tree_id as i64)),
            ("added_at".to_string(), Value::from(self.added_at as i64)),
            ("added_by".to_string(), Value::from(self.added_by as i64)),
            ("deleted_at".to_string(), Self::oi64(&self.deleted_at)),
            ("deleted_by".to_string(), Self::oi64(&self.deleted_by)),
            ("small_id".to_string(), Value::from(self.small_id as i64)),
            ("large_id".to_string(), Value::from(self.large_id as i64)),
        ])
    }

    pub fn is_deleted(&self) -> bool {
        if let Some(value) = self.deleted_at {
            if value > 0 {
                return true;
            }
        }

        false
    }

    pub fn is_visible(&self) -> bool {
        !self.is_deleted() && self.small_id > 0 && self.large_id > 0
    }

    fn oi64(value: &Option<u64>) -> Value {
        match value {
            Some(value) => Value::from(*value as i64),
            _ => Value::Null,
        }
    }
}
