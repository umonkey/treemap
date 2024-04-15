use log::{error, warn};
use std::env;

use crate::types::{Error, Result};

const FILE_FOLDER: &str = "FILE_FOLDER";
const JWT_SECRET: &str = "JWT_SECRET";
const PAYLOAD_SIZE: &str = "PAYLOAD_SIZE";
const S3_BUCKET: &str = "TREEMAP_S3_BUCKET";
const S3_ENDPOINT: &str = "TREEMAP_S3_ENDPOINT";
const S3_REGION: &str = "TREEMAP_S3_REGION";
const SERVER_ADDR: &str = "TREEMAP_ADDR";
const SERVER_PORT: &str = "TREEMAP_PORT";
const SQLITE_PATH: &str = "TREEMAP_SQLITE_PATH";
const WORKERS: &str = "TREEMAP_WORKERS";

const DEFAULT_ADDR: &str = "0.0.0.0";
const DEFAULT_FILE_FOLDER: &str = "var/files";
const DEFAULT_JWT_SECRET: &str = "secret";
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

pub fn get_s3_bucket() -> Result<String> {
    match env::var(S3_BUCKET) {
        Ok(v) => Ok(v),

        Err(_) => {
            error!("Environment variable {} not set, unable to continue. Read more at <https://github.com/umonkey/treemap/wiki/Configuration#s3>", S3_BUCKET);
            Err(Error::EnvNotSet)
        }
    }
}

pub fn get_s3_region() -> Result<String> {
    match env::var(S3_REGION) {
        Ok(v) => Ok(v),

        Err(_) => {
            error!("Environment variable {} not set, unable to continue. Read more at <https://github.com/umonkey/treemap/wiki/Configuration#s3>", S3_REGION);
            Err(Error::EnvNotSet)
        }
    }
}

pub fn get_s3_endpoint() -> Result<String> {
    match env::var(S3_ENDPOINT) {
        Ok(v) => Ok(v),

        Err(_) => {
            error!("Environment variable {} not set, unable to continue. Read more at <https://github.com/umonkey/treemap/wiki/Configuration#s3>", S3_ENDPOINT);
            Err(Error::EnvNotSet)
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
