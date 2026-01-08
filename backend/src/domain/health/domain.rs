use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct GetHealthHandler {
    trees: Arc<TreeRepository>,
}

impl GetHealthHandler {
    pub async fn handle(&self) -> Result<()> {
        // This runs a simple query againast the database.
        // We don't care if it finds anything or not, just that it can connect.
        self.trees.get(12345).await?;

        info!("Health check OK.");
        Ok(())
    }
}

impl Locatable for GetHealthHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
