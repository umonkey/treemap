use crate::domain::tree::TreeRepository;
use crate::services::{Locatable, Locator};
use crate::types::Result;
use log::info;
use std::sync::Arc;

pub struct HealthService {
    trees: Arc<TreeRepository>,
}

impl HealthService {
    pub async fn check(&self) -> Result<()> {
        // This runs a simple query againast the database.
        // We don't care if it finds anything or not, just that it can connect.
        self.trees.get(12345).await?;

        info!("Health check OK.");

        Ok(())
    }
}

impl Locatable for HealthService {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
