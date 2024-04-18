use crate::types::CommentRecord;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PublicCommentInfo {
    pub id: String,
    pub added_at: u64,
    pub message: String,
}

impl PublicCommentInfo {
    pub fn from_record(record: &CommentRecord) -> Self {
        Self {
            id: record.id.to_string(),
            added_at: record.added_at,
            message: record.message.clone(),
        }
    }
}
