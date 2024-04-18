use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AddCommentRequest {
    pub tree_id: u64,
    pub message: String,
    pub user_id: u64,
}
