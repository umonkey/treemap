use crate::services::*;
use crate::types::*;
use log::error;
use serde::Deserialize;

const CONFIG_FILE: &str = "config.toml";

const DEFAULT_WEB_WORKERS: usize = 1;
const DEFAULT_WEB_ADDR: &str = "0.0.0.0";
const DEFAULT_WEB_PORT: u16 = 8000;
const DEFAULT_MAX_UPLOAD_SIZE: usize = 50_485_760;
const DEFAULT_OSM_CHANGESET_SIZE: u64 = 100;

#[derive(Default, Deserialize)]
pub struct ConfigService {
    pub bot_user_id: u64,
    #[serde(default = "default_web_workers")]
    pub web_workers: usize,
    #[serde(default = "default_web_addr")]
    pub web_addr: String,
    #[serde(default = "default_web_port")]
    pub web_port: u16,
    #[serde(default = "default_max_upload_size")]
    pub max_upload_size: usize,
    #[serde(default = "default_osm_changeset_size")]
    pub osm_changeset_size: u64,
    pub osm_activity: Option<String>,
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

fn default_web_addr() -> String {
    DEFAULT_WEB_ADDR.to_string()
}

fn default_web_port() -> u16 {
    DEFAULT_WEB_PORT
}

fn default_max_upload_size() -> usize {
    DEFAULT_MAX_UPLOAD_SIZE
}

fn default_osm_changeset_size() -> u64 {
    DEFAULT_OSM_CHANGESET_SIZE
}
