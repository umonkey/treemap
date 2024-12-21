use crate::services::*;
use crate::types::*;
use log::{error, info};
use std::sync::Arc;

pub struct UpdateTreeAddressHandler {
    db: Arc<dyn DatabaseInterface>,
    http: reqwest::Client,
}

impl UpdateTreeAddressHandler {
    pub async fn handle(&self, tree_id: u64) -> Result<()> {
        if let Ok(Some(tree)) = self.db.get_tree(tree_id).await {
            if let Ok(address) = self.get_address(tree.lat, tree.lon).await {
                info!("Updating tree {} address to: {}", tree_id, address);
            };
        }

        Ok(())
    }

    async fn get_address(&self, lat: f64, lon: f64) -> Result<String> {
        let url = format!(
            "https://nominatim.openstreetmap.org/reverse?format=json&lat={}&lon={}",
            lat, lon
        );

        let response = match self.http.get(&url).send().await {
            Ok(response) => response,

            Err(e) => {
                error!("Error contacting Nominatim: {}", e);
                return Err(Error::AddressNotFound);
            }
        };

        let json = match response.json::<serde_json::Value>().await {
            Ok(json) => json,

            Err(e) => {
                error!("Error parsing Nominatim response: {}", e);
                return Err(Error::AddressNotFound);
            }
        };

        if let Some(address) = json["address"].as_str() {
            Ok(address.to_string())
        } else {
            Err(Error::AddressNotFound)
        }
    }
}

impl Locatable for UpdateTreeAddressHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        let http = reqwest::Client::new();
        Ok(Self { db, http })
    }
}
