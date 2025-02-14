use crate::common::database::repositories::*;
use crate::handlers::GetTreeHandler;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct UpdateTreeStateHandler {
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
    getter: Arc<GetTreeHandler>,
}

impl UpdateTreeStateHandler {
    pub async fn handle(
        &self,
        tree_id: u64,
        value: String,
        user_id: u64,
    ) -> Result<SingleTreeResponse> {
        let tree = self.trees.get(tree_id).await?.ok_or(Error::TreeNotFound)?;

        self.trees
            .update(
                &TreeRecord {
                    state: value.clone(),
                    ..tree
                },
                user_id,
            )
            .await?;

        self.users.increment_update_count(user_id).await?;

        info!(
            "State for tree {} changed to {} by {}.",
            tree_id, value, user_id
        );

        self.getter.handle(tree_id).await
    }
}

impl Locatable for UpdateTreeStateHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
            getter: locator.get::<GetTreeHandler>()?,
        })
    }
}
