use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetUpdatedTreesHandler {
    loader: Arc<TreeListLoader>,
    trees: Arc<TreeRepository>,
}

impl GetUpdatedTreesHandler {
    pub async fn handle(&self, count: u64, skip: u64) -> Result<TreeList> {
        let trees = self.trees.get_recently_updated(count, skip).await?;
        self.loader.load(&trees).await
    }
}

impl Locatable for GetUpdatedTreesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            loader: locator.get::<TreeListLoader>()?,
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
