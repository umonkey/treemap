use crate::services::*;
use crate::types::*;
use log::error;
use serde::Deserialize;

const CONFIG_FILE: &str = "config.toml";

const DEFAULT_WEB_WORKERS: usize = 1;

#[derive(Default, Deserialize)]
pub struct ConfigService {
    pub bot_user_id: u64,
    #[serde(default = "default_web_workers")]
    pub web_workers: usize,
}

impl ConfigService {
    pub fn read() -> Result<Self> {
        let source = std::fs::read_to_string(CONFIG_FILE).map_err(|e| {
            error!("Error reading {}: {}", CONFIG_FILE, e);
            Error::DependencyLoad
        })?;

        let config = toml::from_str(&source).map_err(|e| {
            error!("Error parsing {}: {}", CONFIG_FILE, e);
            Error::DependencyLoad
        })?;

        Ok(config)
    }
}

impl Locatable for ConfigService {
    fn create(_locator: &Locator) -> Result<Self> {
        Self::read()
    }
}

fn default_web_workers() -> usize {
    DEFAULT_WEB_WORKERS
}
