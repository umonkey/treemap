use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTreeCommentsHandler {
    db: Arc<dyn DatabaseInterface>,
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
}

impl GetTreeCommentsHandler {
    pub async fn handle(&self, tree_id: u64) -> Result<CommentList> {
        let comments = self.db.find_comments_by_tree(tree_id).await?;

        let user_ids: Vec<u64> = comments.iter().map(|r| r.added_by).collect();
        let users = self.users.get_multiple(&user_ids).await?;

        let tree_ids: Vec<u64> = comments.iter().map(|r| r.tree_id).collect();
        let trees = self.trees.get_multiple(&tree_ids).await?;

        Ok(CommentList::from_records(&comments, &users, &trees))
    }
}

impl Locatable for GetTreeCommentsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            db: locator.get::<PreferredDatabase>()?.driver(),
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
        })
    }
}
