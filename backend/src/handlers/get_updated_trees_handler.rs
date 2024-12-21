use crate::services::*;
use crate::types::*;
use std::collections::HashSet;
use std::sync::Arc;

pub struct GetUpdatedTreesHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl GetUpdatedTreesHandler {
    pub async fn handle(&self, count: u64, skip: u64) -> Result<TreeList> {
        let trees = self.db.get_updated_trees(count, skip).await?;

        let user_ids: Vec<u64> = trees.iter().map(|t| t.added_by).collect();
        let users = self.load_users(&user_ids).await?;

        Ok(TreeList::from_trees(trees).with_users(&users))
    }

    async fn load_users(&self, user_ids: &[u64]) -> Result<Vec<UserRecord>> {
        let user_ids = HashSet::<u64>::from_iter(user_ids.iter().copied());
        let user_ids: Vec<u64> = user_ids.into_iter().collect();
        let users = self.db.get_users(&user_ids).await?;
        Ok(users)
    }
}

impl Locatable for GetUpdatedTreesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
