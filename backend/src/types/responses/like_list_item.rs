use crate::types::LikeRecord;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LikeListItem {
    pub tree_id: String,
    pub user_id: String,
}

impl LikeListItem {
    pub fn from_record(record: &LikeRecord) -> Self {
        Self {
            tree_id: record.tree_id.to_string(),
            user_id: record.user_id.to_string(),
        }
    }
}
