use crate::domain::tree::TreeRepository;
use crate::services::{Context, Injectable};
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

impl Injectable for HealthService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            trees: Arc::new(ctx.build::<TreeRepository>()?),
        })
    }
}
