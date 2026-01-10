//! This service class enriches comments with tree and user info.

use super::schemas::CommentList;
use crate::common::database::repositories::*;
use crate::domain::comment::Comment;
use crate::domain::tree::TreeRepository;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct CommentLoader {
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
}

impl CommentLoader {
    pub async fn load(&self, comments: &[Comment]) -> Result<CommentList> {
        let user_ids: Vec<u64> = comments.iter().map(|r| r.added_by).collect();
        let users = self.users.get_multiple(&user_ids).await?;

        let tree_ids: Vec<u64> = comments.iter().map(|r| r.tree_id).collect();
        let trees = self.trees.get_multiple(&tree_ids).await?;

        Ok(CommentList::from_records(comments, &users, &trees))
    }
}

impl Locatable for CommentLoader {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
        })
    }
}
