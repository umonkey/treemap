use crate::types::{Attributes, Result};
use crate::utils::de_bool;
use rusqlite::types::Value;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LikeRecord {
    pub tree_id: u64,
    pub user_id: u64,
    #[serde(deserialize_with = "de_bool")]
    pub state: bool,
    pub updated_at: u64,
}

impl LikeRecord {
    #[allow(unused)]
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        attributes.deserialize::<Self>()
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
