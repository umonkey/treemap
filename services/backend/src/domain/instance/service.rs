use super::models::Instance;
use super::repository::InstanceRepository;
use crate::services::{Context, Injectable};
use crate::types::*;
use std::sync::Arc;

pub struct InstanceService {
    instances: Arc<InstanceRepository>,
}

impl InstanceService {
    pub async fn validate_instance(&self, domain: &str) -> Result<Instance> {
        let instance = self
            .instances
            .get_by_domain(domain)
            .await?
            .ok_or(Error::InstanceNotFound)?;

        if !instance.enabled {
            return Err(Error::InstanceDisabled);
        }

        Ok(instance)
    }
}

impl Injectable for InstanceService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            instances: Arc::new(ctx.build::<InstanceRepository>()?),
        })
    }
}
