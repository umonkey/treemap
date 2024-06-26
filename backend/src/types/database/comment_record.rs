/**
 * This is how a single comment is stored in the database.
 */
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct CommentRecord {
    pub id: u64,
    pub tree_id: u64,
    pub added_at: u64,
    pub added_by: u64,
    pub message: String,
}
