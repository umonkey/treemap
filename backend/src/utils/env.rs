use crate::types::{Error, Result};
use log::warn;
use std::env;

const JWT_SECRET: &str = "JWT_SECRET";

const DEFAULT_JWT_SECRET: &str = "secret";

pub fn get_jwt_secret() -> String {
    match env::var(JWT_SECRET) {
        Ok(v) => v,

        Err(_) => {
            warn!("Environment variable {} not set, using default: {}. This is very insecure, only OK for a test environment. Read more at <https://github.com/umonkey/treemap/wiki/Configuration#jwt_secret>", JWT_SECRET, DEFAULT_JWT_SECRET);
            DEFAULT_JWT_SECRET.to_string()
        }
    }
}

pub fn get_osm_client_secret() -> Result<String> {
    env::var("OSM_CLIENT_SECRET").map_err(|_| Error::EnvNotSet)
}

pub fn get_osm_token() -> Result<String> {
    env::var("OSM_TOKEN").map_err(|_| Error::EnvNotSet)
}

pub fn get_app_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
