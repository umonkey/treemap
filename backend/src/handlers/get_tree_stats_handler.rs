use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTreeStatsHandler {
    trees: Arc<TreeRepository>,
}

impl GetTreeStatsHandler {
    pub async fn handle(&self) -> Result<TreeStatsResponse> {
        Ok(TreeStatsResponse {
            count: self.trees.count().await?,
        })
    }
}

impl Locatable for GetTreeStatsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
