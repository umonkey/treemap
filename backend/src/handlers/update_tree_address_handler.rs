//! Update tree address using OSM's Nominatim service.
//!
//! This is executed after a tree is added or updated,
//! and its address is not known, or the coordinates have changed.

use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct UpdateTreeAddressHandler {
    config: Arc<ConfigService>,
    trees: Arc<TreeRepository>,
    nominatim: Arc<NominatimService>,
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
                        self.config.bot_user_id,
                    )
                    .await?;
            };
        }

        Ok(())
    }
}

impl Locatable for UpdateTreeAddressHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            config: locator.get::<ConfigService>()?,
            trees: locator.get::<TreeRepository>()?,
            nominatim: locator.get::<NominatimService>()?,
        })
    }
}
