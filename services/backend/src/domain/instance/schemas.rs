use super::models::Instance;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize)]
pub struct InstanceUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub osm_pull: Option<bool>,
    // An empty string clears the value; an absent or null value keeps the current one.
    pub osm_bot_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InstanceRead {
    pub name: String,
    pub description: Option<String>,
    pub osm_pull: bool,
    pub osm_bot_id: Option<String>,
}

impl From<Instance> for InstanceRead {
    fn from(instance: Instance) -> Self {
        Self {
            name: instance.name,
            description: instance.description,
            osm_pull: instance.osm_pull,
            osm_bot_id: instance.osm_bot_id.map(|id| id.to_string()),
        }
    }
}
