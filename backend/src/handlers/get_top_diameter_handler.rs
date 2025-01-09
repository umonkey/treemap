use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTopDiameterHandler {
    loader: Arc<TreeListLoader>,
    trees: Arc<TreeRepository>,
}

impl GetTopDiameterHandler {
    pub async fn handle(&self) -> Result<TreeList> {
        let trees = self.trees.get_top_diameter(100).await?;
        self.loader.load(&trees).await
    }
}

impl Locatable for GetTopDiameterHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            loader: locator.get::<TreeListLoader>()?,
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
