use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetNewTreesHandler {
    db: Arc<dyn DatabaseInterface>,
    loader: Arc<TreeListLoader>,
}

impl GetNewTreesHandler {
    pub async fn handle(&self, count: u64, skip: u64) -> Result<TreeList> {
        let trees = self.db.get_new_trees(count, skip).await?;
        self.loader.load(&trees).await
    }
}

impl Locatable for GetNewTreesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        let loader = locator.get::<TreeListLoader>()?;
        Ok(Self { db, loader })
    }
}
