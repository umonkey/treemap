//! Comment injector service.
//!
//! Use this whenever you need to create a new comment, with all the added
//! logic that is required, e.g. updating the comment counts everywhere.
//!
//! This is currently being used by the "add comment" endpoint, and the "state
//! change" endpoint which can also add comments implicitly.

use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use log::info;
use std::sync::Arc;

pub struct CommentInjector {
    comments: Arc<CommentRepository>,
    users: Arc<UserRepository>,
    trees: Arc<TreeRepository>,
}

impl CommentInjector {
    pub async fn inject(&self, tree_id: u64, user_id: u64, text: &str) -> Result<u64> {
        let id = get_unique_id()?;
        let now = get_timestamp();

        let comment = CommentRecord {
            id,
            tree_id,
            added_at: now,
            added_by: user_id,
            message: text.to_string(),
        };

        self.comments.add(&comment).await?;

        self.users.increment_comment_count(user_id).await?;

        let comments = self.comments.count_by_tree(tree_id).await?;
        self.trees.update_comment_count(tree_id, comments).await?;

        info!("Comment {id} added to tree {tree_id}");

        Ok(id)
    }
}

impl Locatable for CommentInjector {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            comments: locator.get::<CommentRepository>()?,
            users: locator.get::<UserRepository>()?,
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
