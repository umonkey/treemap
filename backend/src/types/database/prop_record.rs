use crate::types::*;
use crate::infra::database::{Attributes, Value};
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct PropRecord {
    pub id: u64,
    pub tree_id: u64,
    pub added_at: u64,
    pub added_by: u64,
    pub name: String,
    pub value: String,
}

impl PropRecord {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            tree_id: attributes.require_u64("tree_id")?,
            added_at: attributes.require_u64("added_at")?,
            added_by: attributes.require_u64("added_by")?,
            name: attributes.require_string("name")?,
            value: attributes.require_string("value")?,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            ("tree_id".to_string(), Value::from(self.tree_id as i64)),
            ("added_at".to_string(), Value::from(self.added_at as i64)),
            ("added_by".to_string(), Value::from(self.added_by as i64)),
            ("name".to_string(), Value::from(self.name.clone())),
            ("value".to_string(), Value::from(self.value.clone())),
        ])
    }
}
