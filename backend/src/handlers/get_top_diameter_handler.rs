use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTopDiameterHandler {
    db: Arc<dyn DatabaseInterface>,
    loader: Arc<TreeListLoader>,
}

impl GetTopDiameterHandler {
    pub async fn handle(&self) -> Result<TreeList> {
        let trees = self.db.get_top_diameter(100).await?;
        self.loader.load(&trees).await
    }
}

impl Locatable for GetTopDiameterHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        let loader = locator.get::<TreeListLoader>()?;
        Ok(Self { db, loader })
    }
}
