use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTopHeightHandler {
    loader: Arc<TreeListLoader>,
    trees: Arc<TreeRepository>,
}

impl GetTopHeightHandler {
    pub async fn handle(&self) -> Result<TreeList> {
        let trees = self.trees.get_top_height(100).await?;
        self.loader.load(&trees).await
    }
}

impl Locatable for GetTopHeightHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            loader: locator.get::<TreeListLoader>()?,
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
