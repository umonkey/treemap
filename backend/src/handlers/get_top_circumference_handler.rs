use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTopCircumferenceHandler {
    loader: Arc<TreeListLoader>,
    trees: Arc<TreeRepository>,
}

impl GetTopCircumferenceHandler {
    pub async fn handle(&self) -> Result<TreeList> {
        let trees = self.trees.get_top_circumference(100).await?;
        self.loader.load(&trees).await
    }
}

impl Locatable for GetTopCircumferenceHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            loader: locator.get::<TreeListLoader>()?,
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
