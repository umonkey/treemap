//! Update addresses for all trees that don't have one.
//!
//! This is only executed using a dedicated CLI command.

use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::*;
use log::{info, warn};
use std::sync::Arc;

pub struct UpdateTreeAddressesHandler {
    trees: Arc<TreeRepository>,
    nominatim: Arc<NominatimService>,
    user_id: u64,
}

impl UpdateTreeAddressesHandler {
    pub async fn handle(&self) -> Result<()> {
        let trees = self.trees.get_with_no_address().await?;

        for tree in trees {
            self.update_tree_address(&tree).await?;
            self.sleep();
        }

        Ok(())
    }

    // Nominotim's usage policy requires request rate of at most 1 rps.
    // Let's wait twice as much.
    // This is a one time manually ran operation so we aren't in a hurry.
    // @docs https://operations.osmfoundation.org/policies/nominatim/
    fn sleep(&self) {
        let dur = std::time::Duration::from_secs(2);
        std::thread::sleep(dur);
    }

    async fn update_tree_address(&self, tree: &TreeRecord) -> Result<()> {
        let address = match self
            .nominatim
            .get_street_address(tree.lat, tree.lon)
            .await?
        {
            Some(value) => value,

            None => {
                warn!("No address for tree {}.", tree.id);
                return Ok(());
            }
        };

        info!("Updating tree {} address to: {}", tree.id, address);

        self.trees
            .update(
                &TreeRecord {
                    address: Some(address),
                    ..tree.clone()
                },
                self.user_id,
            )
            .await?;

        Ok(())
    }
}

impl Locatable for UpdateTreeAddressesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
            nominatim: locator.get::<NominatimService>()?,
            user_id: get_bot_user_id(),
        })
    }
}
