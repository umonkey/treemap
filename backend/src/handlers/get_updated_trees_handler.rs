use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetUpdatedTreesHandler {
    db: Arc<dyn DatabaseInterface>,
    loader: Arc<TreeListLoader>,
}

impl GetUpdatedTreesHandler {
    pub async fn handle(&self, count: u64, skip: u64) -> Result<TreeList> {
        let trees = self.db.get_updated_trees(count, skip).await?;
        self.loader.load(&trees).await
    }
}

impl Locatable for GetUpdatedTreesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        let loader = locator.get::<TreeListLoader>()?;
        Ok(Self { db, loader })
    }
}
