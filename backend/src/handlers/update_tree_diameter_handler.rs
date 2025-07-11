use crate::common::database::repositories::*;
use crate::handlers::GetTreeHandler;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct UpdateTreeDiameterHandler {
    props: Arc<PropRepository>,
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
    getter: Arc<GetTreeHandler>,
}

impl UpdateTreeDiameterHandler {
    pub async fn handle(
        &self,
        tree_id: u64,
        value: f64,
        user_id: u64,
    ) -> Result<SingleTreeResponse> {
        // Validate that tree exists.
        let tree = self.trees.get(tree_id).await?.ok_or(Error::TreeNotFound)?;

        if tree.diameter == Some(value) {
            // If the diameter is the same, we still need to store the property update.
            self.props
                .add(&PropRecord {
                    tree_id,
                    name: "diameter".to_string(),
                    value: value.to_string(),
                    added_by: user_id,
                    ..Default::default()
                })
                .await?;
        }

        self.trees
            .update(
                &TreeRecord {
                    diameter: Some(value),
                    ..tree
                },
                user_id,
            )
            .await?;

        self.users.increment_update_count(user_id).await?;

        info!("Diameter for tree {tree_id} changed to {value} by {user_id}.");

        self.getter.handle(tree_id).await
    }
}

impl Locatable for UpdateTreeDiameterHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            props: locator.get::<PropRepository>()?,
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
            getter: locator.get::<GetTreeHandler>()?,
        })
    }
}
