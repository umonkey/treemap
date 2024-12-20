use crate::types::CommentRecord;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PublicCommentInfo {
    pub id: String,
    pub tree_id: String,
    pub added_at: u64,
    pub added_by: String,
    pub message: String,
}

impl PublicCommentInfo {
    pub fn from_record(record: &CommentRecord) -> Self {
        Self {
            id: record.id.to_string(),
            tree_id: record.tree_id.to_string(),
            added_at: record.added_at,
            added_by: record.added_by.to_string(),
            message: record.message.clone(),
        }
    }
}
