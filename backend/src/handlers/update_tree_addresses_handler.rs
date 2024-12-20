use crate::services::*;
use crate::types::*;
use log::{info, warn};
use std::sync::Arc;

pub struct UpdateTreeAddressesHandler {
    db: Arc<dyn DatabaseInterface>,
    nominatim: Arc<NominatimService>,
}

impl UpdateTreeAddressesHandler {
    pub async fn handle(&self) -> Result<()> {
        let trees = self.db.find_trees_with_no_address().await?;

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

        self.db
            .update_tree(&TreeRecord {
                address: Some(address),
                ..tree.clone()
            })
            .await?;

        Ok(())
    }
}

impl Locatable for UpdateTreeAddressesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        let nominatim = locator.get::<NominatimService>()?;
        Ok(Self { db, nominatim })
    }
}
