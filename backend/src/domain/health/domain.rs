use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct GetHealthHandler {
    trees: Arc<TreeRepository>,
}

impl GetHealthHandler {
    pub async fn handle(&self) -> Result<TreeStatsResponse> {
        let count = self.trees.count().await?;

        info!("Serving tree stats (health check ok).");

        Ok(TreeStatsResponse { count })
    }
}

impl Locatable for GetHealthHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
