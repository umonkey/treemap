use crate::services::Database;
use crate::types::{CommentRecord, Result};
use std::sync::Arc;

pub struct CommentsService {
    db: Arc<dyn Database>,
}

impl CommentsService {
    pub fn new(db: &Arc<dyn Database>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn get_tree_comments(&self, tree_id: u64) -> Result<Vec<CommentRecord>> {
        let comments = self.db.find_comments_by_tree(tree_id).await?;
        Ok(comments)
    }
}
