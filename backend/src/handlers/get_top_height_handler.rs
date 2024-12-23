use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTopHeightHandler {
    db: Arc<dyn DatabaseInterface>,
    loader: Arc<TreeListLoader>,
}

impl GetTopHeightHandler {
    pub async fn handle(&self) -> Result<TreeList> {
        let trees = self.db.get_top_height(100).await?;
        self.loader.load(&trees).await
    }
}

impl Locatable for GetTopHeightHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        let loader = locator.get::<TreeListLoader>()?;
        Ok(Self { db, loader })
    }
}
