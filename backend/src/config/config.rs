use crate::services::{Locatable, Locator};
use crate::types::{Error, Result};
use log::error;
use serde::Deserialize;
use std::fs;
use std::io::Read;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct Config {
    pub bot_user_id: Option<u64>,
    pub file_folder: Option<String>,
    pub jwt_secret: Option<String>,
    pub osm_changeset_size: Option<u64>,
    pub overpass_endpoint: Option<String>,
    pub overpass_query: Option<String>,
    pub payload_size: Option<usize>,
    pub server_addr: Option<String>,
    pub server_port: Option<u16>,
    pub sqlite_path: Option<String>,
    pub workers: Option<usize>,
    pub osm_client_id: Option<String>,
    pub osm_client_secret: Option<String>,
    pub osm_redirect_uri: Option<String>,
    pub osm_token: Option<String>,
    pub osm_hashtag: Option<String>,
    pub osm_activity: Option<String>,
}

impl Config {
    pub fn new() -> Result<Self> {
        Self::from_file("config.toml")
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let mut file = fs::File::open("config.toml").map_err(|e| {
            error!("Error opening {}: {}", path, e);
            Error::Config
        })?;

        let mut contents = String::new();

        file.read_to_string(&mut contents).map_err(|e| {
            error!("Error reading {}: {}", path, e);
            Error::Config
        })?;

        Self::from_string(&contents)
    }

    pub fn from_string(contents: &str) -> Result<Self> {
        toml::from_str(contents).map_err(|e| {
            error!("Error parsing config string: {}", e);
            Error::Config
        })
    }
}

impl Locatable for Config {
    fn create(_locator: &Locator) -> Result<Self> {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing() {
        let file = Config::from_file("missing.toml");
        assert!(file.is_err());
    }

    #[test]
    fn test_ok() {
        let file = include_str!("./fixtures/config.toml");
        let config = Config::from_string(file).expect("Failed to parse config");

        assert_eq!(config.bot_user_id, Some(123456789));
    }
}
