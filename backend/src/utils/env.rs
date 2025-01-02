use log::{error, warn};
use std::env;

use crate::types::{Error, Result};

const BOT_USER_ID: &str = "BOT_USER_ID";
const FILE_FOLDER: &str = "FILE_FOLDER";
const JWT_SECRET: &str = "JWT_SECRET";
const OVERPASS_ENDPOINT: &str = "TREEMAP_OVERPASS_ENDPOINT";
const OVERPASS_QUERY: &str = "TREEMAP_OVERPASS_QUERY";
const PAYLOAD_SIZE: &str = "PAYLOAD_SIZE";
const SERVER_ADDR: &str = "TREEMAP_ADDR";
const SERVER_PORT: &str = "TREEMAP_PORT";
const SQLITE_PATH: &str = "TREEMAP_SQLITE_PATH";
const WORKERS: &str = "TREEMAP_WORKERS";

const DEFAULT_ADDR: &str = "0.0.0.0";
const DEFAULT_BOT_USER_ID: u64 = 0;
const DEFAULT_FILE_FOLDER: &str = "var/files";
const DEFAULT_JWT_SECRET: &str = "secret";
const DEFAULT_OVERPASS_ENDPONT: &str = "https://overpass-api.de/api/interpreter";
const DEFAULT_OVERPASS_QUERY: &str =
    "[out:json];node[natural=tree](40.052848, 44.294472, 40.300476, 44.807396);out;";
const DEFAULT_PAYLOAD_SIZE: usize = 50_485_760;
const DEFAULT_PORT: u16 = 8000;
const DEFAULT_WORKERS: usize = 1;

pub fn get_sqlite_path() -> Result<String> {
    match env::var(SQLITE_PATH) {
        Ok(v) => Ok(v),

        Err(_) => {
            error!("Environment variable {} not set, cannot connect to the database. Read more at <https://github.com/umonkey/treemap/wiki/Configuration#sqlite_path>", SQLITE_PATH);
            Err(Error::EnvNotSet)
        }
    }
}

pub fn get_workers() -> usize {
    match env::var(WORKERS) {
        Ok(v) => v.parse::<usize>().unwrap_or(DEFAULT_WORKERS),
        Err(_) => {
            warn!(
                "Environment variable {} not set, using default {}.",
                WORKERS, DEFAULT_WORKERS
            );
            DEFAULT_WORKERS
        }
    }
}

pub fn get_server_addr() -> String {
    match env::var(SERVER_ADDR) {
        Ok(v) => v,

        Err(_) => {
            warn!(
                "Environment variable {} not set, using default: {}.",
                SERVER_ADDR, DEFAULT_ADDR
            );
            DEFAULT_ADDR.to_string()
        }
    }
}

pub fn get_server_port() -> u16 {
    match env::var(SERVER_PORT) {
        Ok(v) => v.parse::<u16>().unwrap_or(DEFAULT_PORT),

        Err(_) => {
            warn!(
                "Environment variable {} not set, using default {}.",
                SERVER_PORT, DEFAULT_PORT
            );
            DEFAULT_PORT
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

pub fn get_payload_size() -> usize {
    match env::var(PAYLOAD_SIZE) {
        Ok(v) => v.parse::<usize>().unwrap_or(DEFAULT_PAYLOAD_SIZE),

        Err(_) => {
            warn!(
                "Environment variable {} not set, using default {}.",
                PAYLOAD_SIZE, DEFAULT_PAYLOAD_SIZE
            );
            DEFAULT_PAYLOAD_SIZE
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

pub fn get_bot_user_id() -> u64 {
    match env::var(BOT_USER_ID) {
        Ok(v) => v.parse::<u64>().unwrap_or(DEFAULT_BOT_USER_ID),
        Err(_) => {
            warn!(
                "Environment variable {} not set, using default {}.",
                BOT_USER_ID, DEFAULT_BOT_USER_ID
            );
            DEFAULT_BOT_USER_ID
        }
    }
}
