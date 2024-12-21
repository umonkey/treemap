use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct UnlikeTreeHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl UnlikeTreeHandler {
    pub async fn handle(&self, tree_id: u64, user_id: u64) -> Result<()> {
        self.db.unlike_tree(tree_id, user_id).await?;
        Ok(())
    }
}

impl Locatable for UnlikeTreeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
