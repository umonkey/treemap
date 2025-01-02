use crate::services::*;
use crate::types::*;
use crate::utils::*;
use log::info;
use std::sync::Arc;

pub struct UpdateTreeAddressHandler {
    db: Arc<dyn DatabaseInterface>,
    nominatim: Arc<NominatimService>,
    user_id: u64,
}

impl UpdateTreeAddressHandler {
    pub async fn handle(&self, tree_id: u64) -> Result<()> {
        if let Ok(Some(tree)) = self.db.get_tree(tree_id).await {
            if let Ok(Some(address)) = self.nominatim.get_street_address(tree.lat, tree.lon).await {
                info!("Updating tree {} address to: {}", tree_id, address);

                self.db
                    .update_tree(
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
        let db = locator.get::<PreferredDatabase>()?.driver();
        let nominatim = locator.get::<NominatimService>()?;
        let user_id = get_bot_user_id();
        Ok(Self {
            db,
            nominatim,
            user_id,
        })
    }
}
