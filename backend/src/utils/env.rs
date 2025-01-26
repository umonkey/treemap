use log::{error, warn};
use std::env;

use crate::types::{Error, Result};

const FILE_FOLDER: &str = "FILE_FOLDER";
const JWT_SECRET: &str = "JWT_SECRET";
const OVERPASS_ENDPOINT: &str = "TREEMAP_OVERPASS_ENDPOINT";
const OVERPASS_QUERY: &str = "TREEMAP_OVERPASS_QUERY";
const SQLITE_PATH: &str = "TREEMAP_SQLITE_PATH";

const DEFAULT_FILE_FOLDER: &str = "var/files";
const DEFAULT_JWT_SECRET: &str = "secret";
const DEFAULT_OVERPASS_ENDPONT: &str = "https://overpass-api.de/api/interpreter";
const DEFAULT_OVERPASS_QUERY: &str =
    "[out:json];node[natural=tree](40.052848, 44.294472, 40.300476, 44.807396);out;";

pub fn get_sqlite_path() -> Result<String> {
    match env::var(SQLITE_PATH) {
        Ok(v) => Ok(v),

        Err(_) => {
            error!("Environment variable {} not set, cannot connect to the database. Read more at <https://github.com/umonkey/treemap/wiki/Configuration#sqlite_path>", SQLITE_PATH);
            Err(Error::EnvNotSet)
        }
    }
}

pub fn get_jwt_secret() -> String {
    match env::var(JWT_SECRET) {
        Ok(v) => v,

        Err(_) => {
            warn!("Environment variable {} not set, using default: {}. This is very insecure, only OK for a test environment. Read more at <https://github.com/umonkey/treemap/wiki/Configuration#jwt_secret>", JWT_SECRET, DEFAULT_JWT_SECRET);
            DEFAULT_JWT_SECRET.to_string()
        }
    }
}

pub fn get_file_folder() -> String {
    match env::var(FILE_FOLDER) {
        Ok(v) => v,

        Err(_) => {
            warn!("Environment variable {} not set, using default: {}. Read more at <https://github.com/umonkey/treemap/wiki/Configuration#jwt_secret>", FILE_FOLDER, DEFAULT_FILE_FOLDER);
            DEFAULT_FILE_FOLDER.to_string()
        }
    }
}

pub fn get_overpass_endpoint() -> String {
    match env::var(OVERPASS_ENDPOINT) {
        Ok(v) => v,

        Err(_) => {
            warn!("Environment variable {} not set, using default: {}. Read more at <https://github.com/umonkey/treemap/wiki/Configuration#overpass_endpoint>", OVERPASS_ENDPOINT, DEFAULT_OVERPASS_ENDPONT);
            DEFAULT_OVERPASS_ENDPONT.to_string()
        }
    }
}

pub fn get_overpass_query() -> String {
    match env::var(OVERPASS_QUERY) {
        Ok(v) => v,

        Err(_) => {
            warn!("Environment variable {} not set, using default: {}. Read more at <https://github.com/umonkey/treemap/wiki/Configuration#overpass_query>", OVERPASS_QUERY, DEFAULT_OVERPASS_QUERY);
            DEFAULT_OVERPASS_QUERY.to_string()
        }
    }
}

pub fn get_osm_client_id() -> Result<String> {
    env::var("OSM_CLIENT_ID").map_err(|_| Error::EnvNotSet)
}

pub fn get_osm_client_secret() -> Result<String> {
    env::var("OSM_CLIENT_SECRET").map_err(|_| Error::EnvNotSet)
}

pub fn get_osm_redirect_uri() -> Result<String> {
    env::var("OSM_REDIRECT_URI").map_err(|_| Error::EnvNotSet)
}

pub fn get_osm_token() -> Result<String> {
    env::var("OSM_TOKEN").map_err(|_| Error::EnvNotSet)
}

pub fn get_osm_hashtag() -> Result<String> {
    env::var("OSM_HASHTAG").map_err(|_| Error::EnvNotSet)
}

pub fn get_osm_activity() -> Result<String> {
    env::var("OSM_ACTIVITY").map_err(|_| Error::EnvNotSet)
}

pub fn get_dry_run() -> Result<bool> {
    if let Ok(value) = env::var("DRY") {
        return Ok(value == "yes");
    }

    Ok(false)
}

pub fn get_app_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
