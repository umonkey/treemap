use crate::types::Result;
use crate::infra::database::{Attributes, Value};
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct LikeRecord {
    pub tree_id: u64,
    pub user_id: u64,
    pub state: bool,
    pub updated_at: u64,
}

impl LikeRecord {
    #[allow(unused)]
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            tree_id: attributes.require_u64("tree_id")?,
            user_id: attributes.require_u64("user_id")?,
            state: attributes.require_i64("state")? != 0,
            updated_at: attributes.require_u64("updated_at")?,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("tree_id".to_string(), Value::from(self.tree_id as i64)),
            ("user_id".to_string(), Value::from(self.user_id as i64)),
            (
                "state".to_string(),
                Value::from(if self.state { 1 } else { 0 }),
            ),
            (
                "updated_at".to_string(),
                Value::from(self.updated_at as i64),
            ),
        ])
    }
}
