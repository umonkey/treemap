//! Update current height of a tree.
//! Note that this performa live database update.

use crate::common::database::repositories::*;
use crate::handlers::GetTreeHandler;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct UpdateTreeHeightHandler {
    getter: Arc<GetTreeHandler>,
    props: Arc<PropRepository>,
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
}

impl UpdateTreeHeightHandler {
    pub async fn handle(
        &self,
        tree_id: u64,
        value: f64,
        user_id: u64,
    ) -> Result<SingleTreeResponse> {
        // Make sure the tree exists.
        let tree = self.trees.get(tree_id).await?.ok_or(Error::TreeNotFound)?;

        if tree.height == Some(value) {
            // If the height is the same, we still need to store the property update.
            self.props
                .add(&PropRecord {
                    tree_id,
                    name: "height".to_string(),
                    value: value.to_string(),
                    added_by: user_id,
                    ..Default::default()
                })
                .await?;
        }

        self.trees
            .update(
                &TreeRecord {
                    height: Some(value),
                    ..tree
                },
                user_id,
            )
            .await?;

        self.users.increment_update_count(user_id).await?;

        info!(
            "Height for tree {} changed to {} by {}.",
            tree_id, value, user_id
        );

        self.getter.handle(tree_id).await
    }
}

impl Locatable for UpdateTreeHeightHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            props: locator.get::<PropRepository>()?,
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
            getter: locator.get::<GetTreeHandler>()?,
        })
    }
}
