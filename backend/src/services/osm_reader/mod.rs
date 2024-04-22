/**
 * This module reads data from OSM and puts it into the database.
 */

use log::info;

use crate::types::Result;
use crate::services::OverpassClient;

pub struct OsmReaderService {
    overpass_client: OverpassClient,
}

impl OsmReaderService {
    pub async fn init() -> Result<Self> {
        Ok(Self {
            overpass_client: OverpassClient::init(),
        })
    }

    pub async fn run(&self) {
        info!("Running OSM reader service.");

        self.overpass_client.query().await.unwrap();
    }
}
