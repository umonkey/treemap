use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct UpdateTreeAddressHandler {
    db: Arc<dyn DatabaseInterface>,
    nominatim: Arc<NominatimService>,
}

impl UpdateTreeAddressHandler {
    pub async fn handle(&self, tree_id: u64) -> Result<()> {
        if let Ok(Some(tree)) = self.db.get_tree(tree_id).await {
            if let Ok(Some(address)) = self.nominatim.get_street_address(tree.lat, tree.lon).await {
                info!("Updating tree {} address to: {}", tree_id, address);

                self.db
                    .update_tree(&TreeRecord {
                        address: Some(address),
                        ..tree.clone()
                    })
                    .await?;
            };
        }

        Ok(())
    }
}

impl Locatable for UpdateTreeAddressHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        let nominatim = locator.get::<NominatimService>()?;
        Ok(Self { db, nominatim })
    }
}
