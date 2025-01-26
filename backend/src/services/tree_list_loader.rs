use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::unique_ids;
use std::sync::Arc;

pub struct TreeListLoader {
    users: Arc<UserRepository>,
}

impl TreeListLoader {
    pub async fn load(&self, trees: &[TreeRecord]) -> Result<TreeList> {
        let user_ids: Vec<u64> = trees.iter().map(|t| t.added_by).collect();
        let users = self.load_users(&user_ids).await?;

        Ok(TreeList::from_trees(trees).with_users(&users))
    }

    async fn load_users(&self, user_ids: &[u64]) -> Result<Vec<UserRecord>> {
        let ids = unique_ids(user_ids);
        let users = self.users.get_multiple(&ids).await?;
        Ok(users)
    }
}

impl Locatable for TreeListLoader {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            users: locator.get::<UserRepository>()?,
        })
    }
}
