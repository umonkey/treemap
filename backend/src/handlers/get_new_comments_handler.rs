use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetNewCommentsHandler {
    comments: Arc<CommentRepository>,
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
}

impl GetNewCommentsHandler {
    pub async fn handle(&self, limit: u64) -> Result<CommentList> {
        let comments = self.comments.find_recent(limit).await?;

        let user_ids: Vec<u64> = comments.iter().map(|r| r.added_by).collect();
        let users = self.users.get_multiple(&user_ids).await?;

        let tree_ids: Vec<u64> = comments.iter().map(|r| r.tree_id).collect();
        let trees = self.trees.get_multiple(&tree_ids).await?;

        Ok(CommentList::from_records(&comments, &users, &trees))
    }
}

impl Locatable for GetNewCommentsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            comments: locator.get::<CommentRepository>()?,
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
        })
    }
}
