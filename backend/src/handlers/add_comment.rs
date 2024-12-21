use crate::services::Database;
use crate::services::SqliteDatabase;
use crate::services::{Locatable, Locator};
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use log::info;
use std::sync::Arc;

pub struct AddCommentHandler {
    db: Arc<dyn Database + Send + Sync>,
}

impl AddCommentHandler {
    pub fn new(db: Arc<SqliteDatabase>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, req: AddCommentRequest) -> Result<()> {
        let id = get_unique_id()?;
        let now = get_timestamp();

        let comment = CommentRecord {
            id,
            tree_id: req.tree_id,
            added_at: now,
            added_by: req.user_id,
            message: req.message.to_string(),
        };

        self.db.add_comment(&comment).await?;

        info!("Comment {} added to tree {}", id, req.tree_id);

        Ok(())
    }
}

impl Locatable for AddCommentHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<SqliteDatabase>()?;
        Ok(Self::new(db))
    }
}
