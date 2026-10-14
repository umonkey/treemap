use super::models::Instance;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct InstanceUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InstanceRead {
    pub name: String,
    pub description: Option<String>,
}

impl From<Instance> for InstanceRead {
    fn from(instance: Instance) -> Self {
        Self {
            name: instance.name,
            description: instance.description,
        }
    }
}
