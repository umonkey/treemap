use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use log::info;
use std::sync::Arc;

pub struct AddCommentHandler {
    comments: Arc<CommentRepository>,
    users: Arc<UserRepository>,
    trees: Arc<TreeRepository>,
}

impl AddCommentHandler {
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

        self.comments.add(&comment).await?;

        self.users.increment_comment_count(req.user_id).await?;

        let comments = self.comments.count_by_tree(req.tree_id).await?;
        self.trees
            .update_comment_count(req.tree_id, comments)
            .await?;

        info!("Comment {} added to tree {}", id, req.tree_id);

        Ok(())
    }
}

impl Locatable for AddCommentHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            comments: locator.get::<CommentRepository>()?,
            users: locator.get::<UserRepository>()?,
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
