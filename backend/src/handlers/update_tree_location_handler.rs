//! Update current location of a tree.
//! Note that this performs live database update.

use crate::common::database::repositories::*;
use crate::handlers::GetTreeHandler;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct UpdateTreeLocationHandler {
    getter: Arc<GetTreeHandler>,
    props: Arc<PropRepository>,
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
}

impl UpdateTreeLocationHandler {
    pub async fn handle(
        &self,
        tree_id: u64,
        lat: f64,
        lon: f64,
        user_id: u64,
    ) -> Result<SingleTreeResponse> {
        // Make sure the tree exists.
        let tree = self.trees.get(tree_id).await?.ok_or(Error::TreeNotFound)?;

        if tree.lat == lat && tree.lon == lon {
            // If the height is the same, we still need to store the property update.
            self.props
                .add(&PropRecord {
                    tree_id,
                    name: "location".to_string(),
                    value: format!("{},{}", lat, lon).to_string(),
                    added_by: user_id,
                    ..Default::default()
                })
                .await?;
        }

        self.trees
            .update(&TreeRecord { lat, lon, ..tree }, user_id)
            .await?;

        self.users.increment_update_count(user_id).await?;

        info!(
            "Location for tree {} changed to {},{} by {}.",
            tree_id, lat, lon, user_id
        );

        self.getter.handle(tree_id).await
    }
}

impl Locatable for UpdateTreeLocationHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            props: locator.get::<PropRepository>()?,
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
            getter: locator.get::<GetTreeHandler>()?,
        })
    }
}
