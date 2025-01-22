//! Update tree address using OSM's Nominatim service.
//!
//! This is executed after a tree is added or updated,
//! and its address is not known, or the coordinates have changed.

use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::*;
use log::info;
use std::sync::Arc;

pub struct UpdateTreeAddressHandler {
    trees: Arc<TreeRepository>,
    nominatim: Arc<NominatimService>,
    user_id: u64,
}

impl UpdateTreeAddressHandler {
    pub async fn handle(&self, tree_id: u64) -> Result<()> {
        if let Ok(Some(tree)) = self.trees.get(tree_id).await {
            if let Ok(Some(address)) = self.nominatim.get_street_address(tree.lat, tree.lon).await {
                info!("Updating tree {} address to: {}", tree_id, address);

                self.trees
                    .update(
                        &TreeRecord {
                            address: Some(address),
                            ..tree.clone()
                        },
                        self.user_id,
                    )
                    .await?;
            };
        }

        Ok(())
    }
}

impl Locatable for UpdateTreeAddressHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let trees = locator.get::<TreeRepository>()?;
        let nominatim = locator.get::<NominatimService>()?;
        let user_id = get_bot_user_id();
        Ok(Self {
            trees,
            nominatim,
            user_id,
        })
    }
}
