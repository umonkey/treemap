//! Reads configuration from the config.toml file.
//!
//! The default config file name is `config.toml`, but it can be overridden using the
//! `TREEMAP_CONFIG` environment variable, which is normally only used by unit tests.

use crate::services::{Locatable, Locator};
use crate::types::{Error, Result};
use log::{error, warn};
use serde::Deserialize;
use std::fs;
use std::io::Read;

#[allow(unused)]
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default = "default_bot_user_id")]
    pub bot_user_id: u64,

    #[serde(default = "default_file_folder")]
    pub file_folder: String,

    pub jwt_secret: Option<String>,

    #[serde(default = "default_osm_changeset_size")]
    pub osm_changeset_size: u64,

    pub osm_client_id: Option<String>,
    pub osm_hashtag: Option<String>,
    pub osm_activity: Option<String>,
    pub osm_redirect_uri: Option<String>,

    #[serde(default = "default_overpass_endpoint")]
    pub overpass_endpoint: String,

    #[serde(default = "default_overpass_query")]
    pub overpass_query: String,

    #[serde(default = "default_payload_size")]
    pub payload_size: usize,

    #[serde(default = "default_server_addr")]
    pub server_addr: String,

    #[serde(default = "default_server_port")]
    pub server_port: u16,

    #[serde(default = "default_sqlite_path")]
    pub sqlite_path: String,

    #[serde(default = "default_secrets_path")]
    pub secrets_path: String,

    // The number of web worker threads to spawn.
    #[serde(default = "default_workers")]
    pub workers: usize,
}

impl Config {
    pub fn new() -> Result<Self> {
        Self::from_default_file()
    }

    #[cfg(test)]
    pub fn from_default_file() -> Result<Self> {
        Self::from_file("tests/config.toml")
    }

    #[cfg(not(test))]
    pub fn from_default_file() -> Result<Self> {
        let path: String = match std::env::var("TREEMAP_CONFIG") {
            Ok(v) => v,
            Err(_) => "config.toml".to_string(),
        };

        Self::from_file(&path)
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let mut file = match fs::File::open(path) {
            Ok(file) => file,

            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    warn!("Config file {path} not found, using default config.");
                    return Self::from_string("");
                }

                _ => {
                    error!("Error opening {path}: {e:?}");
                    return Err(Error::Config);
                }
            },
        };

        let mut contents = String::new();

        file.read_to_string(&mut contents).map_err(|e| {
            error!("Error reading {path}: {e}");
            Error::Config
        })?;

        Self::from_string(&contents)
    }

    pub fn from_string(contents: &str) -> Result<Self> {
        let data = toml::from_str(contents).map_err(|e| {
            error!("Error parsing config string: {e}");
            Error::Config
        })?;

        Ok(data)
    }
}

impl Locatable for Config {
    fn create(_locator: &Locator) -> Result<Self> {
        Self::new()
    }
}

fn default_server_addr() -> String {
    "0.0.0.0".to_string()
}

fn default_server_port() -> u16 {
    8000
}

fn default_workers() -> usize {
    1
}

fn default_payload_size() -> usize {
    50_485_760
}

fn default_file_folder() -> String {
    "var/files".to_string()
}

fn default_bot_user_id() -> u64 {
    0
}

fn default_osm_changeset_size() -> u64 {
    1
}

fn default_overpass_endpoint() -> String {
    "https://overpass-api.de/api/interpreter".to_string()
}

fn default_overpass_query() -> String {
    "[out:json];node[natural=tree](40.052848, 44.294472, 40.300476, 44.807396);out;".to_string()
}

fn default_sqlite_path() -> String {
    "var/database.sqlite".to_string()
}

fn default_secrets_path() -> String {
    ".secrets".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing() {
        let file = Config::from_file("missing.toml").expect("Error reading default config file.");
        assert_eq!("0.0.0.0", file.server_addr);
    }

    #[test]
    fn test_ok() {
        let file = include_str!("./fixtures/config.toml");
        let config = Config::from_string(file).expect("Failed to parse config");

        assert_eq!(config.bot_user_id, 123456789);
    }
}
