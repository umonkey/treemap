use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct GetTreeStatsHandler {
    trees: Arc<TreeRepository>,
}

impl GetTreeStatsHandler {
    pub async fn handle(&self) -> Result<TreeStatsResponse> {
        let count = self.trees.count().await?;

        info!("Serving tree stats (health check ok).");

        Ok(TreeStatsResponse { count })
    }
}

impl Locatable for GetTreeStatsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
