use super::repository::CommentRepository;
use crate::domain::comment::Comment;
use crate::domain::tree::TreeRepository;
use crate::domain::user::UserRepository;
use crate::services::{Locatable, Locator};
use crate::types::Result;
use crate::utils::{get_timestamp, get_unique_id};
use log::info;
use std::sync::Arc;

pub struct CommentService {
    comments: Arc<CommentRepository>,
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
}

impl CommentService {
    pub async fn get_tree_comments(&self, tree_id: u64) -> Result<Vec<Comment>> {
        self.comments.find_by_tree(tree_id).await
    }

    pub async fn add_comment(&self, tree_id: u64, user_id: u64, message: &str) -> Result<Comment> {
        let id = get_unique_id()?;
        let now = get_timestamp();

        let comment = Comment {
            id,
            tree_id,
            added_at: now,
            added_by: user_id,
            message: message.to_string(),
        };

        self.comments.add(&comment).await?;

        self.users.increment_comment_count(user_id).await?;

        let comments = self.comments.count_by_tree(tree_id).await?;
        self.trees.update_comment_count(tree_id, comments).await?;

        info!("Comment {id} added to tree {tree_id}");

        Ok(comment)
    }

    pub async fn get_new(&self, limit: u64) -> Result<Vec<Comment>> {
        self.comments.find_recent(limit).await
    }
}

impl Locatable for CommentService {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            comments: locator.get::<CommentRepository>()?,
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
        })
    }
}
