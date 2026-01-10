//! Update addresses for all trees that don't have one.
//!
//! This is only executed using a dedicated CLI command.

use crate::domain::tree::Tree;
use crate::domain::tree::TreeRepository;
use crate::infra::config::Config;
use crate::infra::nominatim::NominatimClient;
use crate::services::{Locatable, Locator};
use crate::types::Result;
use log::{info, warn};
use std::sync::Arc;

pub struct UpdateTreeAddressesHandler {
    trees: Arc<TreeRepository>,
    nominatim: Arc<NominatimClient>,
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

    async fn update_tree_address(&self, tree: &Tree) -> Result<()> {
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
                &Tree {
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
        let config = locator.get::<Config>()?;

        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
            nominatim: locator.get::<NominatimClient>()?,
            user_id: config.bot_user_id,
        })
    }
}
