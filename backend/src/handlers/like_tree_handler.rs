use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct LikeTreeHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl LikeTreeHandler {
    pub async fn handle(&self, tree_id: u64, user_id: u64) -> Result<()> {
        self.db.like_tree(tree_id, user_id).await?;
        Ok(())
    }
}

impl Locatable for LikeTreeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
