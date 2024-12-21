use crate::services::Database;
use crate::services::SqliteDatabase;
use crate::services::{Locatable, Locator};
use crate::types::*;
use std::sync::Arc;

pub struct GetNewCommentsHandler {
    db: Arc<dyn Database + Send + Sync>,
}

impl GetNewCommentsHandler {
    pub fn new(db: Arc<SqliteDatabase>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, limit: u64) -> Result<CommentList> {
        let comments = self.db.find_recent_comments(limit).await?;

        let user_ids: Vec<u64> = comments.iter().map(|r| r.added_by).collect();
        let users = self.db.get_users(&user_ids).await?;

        let tree_ids: Vec<u64> = comments.iter().map(|r| r.tree_id).collect();
        let trees = self.db.get_trees_by_ids(&tree_ids).await?;

        Ok(CommentList::from_records(&comments, &users, &trees))
    }
}

impl Locatable for GetNewCommentsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<SqliteDatabase>()?;
        Ok(Self::new(db))
    }
}
