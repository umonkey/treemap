use crate::infra::osm::OsmClient;
use crate::services::*;
use crate::types::*;
use log::{debug, info};
use std::sync::Arc;

pub struct LoginOsmHandler {
    osm: Arc<OsmClient>,
}

impl LoginOsmHandler {
    pub async fn handle(&self, code: String) -> Result<()> {
        debug!("Authenticating an OSM user.");

        let token = self.osm.get_token(&code).await?;

        info!("OSM token: {token}");

        Ok(())
    }
}

impl Locatable for LoginOsmHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            osm: locator.get::<OsmClient>()?,
        })
    }
}
