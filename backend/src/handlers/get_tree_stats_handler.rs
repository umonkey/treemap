use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTreeStatsHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl GetTreeStatsHandler {
    pub async fn handle(&self) -> Result<TreeStatsResponse> {
        Ok(TreeStatsResponse {
            count: self.db.count_trees().await?,
        })
    }
}

impl Locatable for GetTreeStatsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
