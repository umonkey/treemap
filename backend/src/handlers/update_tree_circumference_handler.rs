use crate::common::database::repositories::*;
use crate::handlers::GetTreeHandler;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct UpdateTreeCircumferenceHandler {
    trees: Arc<TreeRepository>,
    getter: Arc<GetTreeHandler>,
}

impl UpdateTreeCircumferenceHandler {
    pub async fn handle(
        &self,
        tree_id: u64,
        value: f64,
        user_id: u64,
    ) -> Result<SingleTreeResponse> {
        let tree = self.trees.get(tree_id).await?;

        self.trees
            .update(
                &TreeRecord {
                    circumference: Some(value),
                    ..tree
                },
                user_id,
            )
            .await?;

        info!(
            "Circumference for tree {} changed to {} by {}.",
            tree_id, value, user_id
        );

        self.getter.handle(tree_id).await
    }
}

impl Locatable for UpdateTreeCircumferenceHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let trees = locator.get::<TreeRepository>()?;
        let getter = locator.get::<GetTreeHandler>()?;
        Ok(Self { trees, getter })
    }
}
