use log::{error, warn};
use std::env;

use crate::Result;
use crate::errors::Error;

const SQLITE_PATH: &str = "TREEMAP_SQLITE_PATH";
const SERVER_ADDR: &str = "TREEMAP_ADDR";
const SERVER_PORT: &str = "TREEMAP_PORT";
const WORKERS: &str = "TREEMAP_WORKERS";

const DEFAULT_WORKERS: usize = 1;
const DEFAULT_ADDR: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8000;


pub fn get_sqlite_path() -> Result<String> {
    match env::var(SQLITE_PATH) {
        Ok(v) => Ok(v),

        Err(_) => {
            error!("Environment variable {} not set, cannot connect to the database. Read more at <https://github.com/umonkey/treemap/wiki/Configuration#sqlite_path>", SQLITE_PATH);
            Err(Error::EnvNotSet)
        },
    }
}

pub fn get_workers() -> usize {
    match env::var(WORKERS) {
        Ok(v) => v.parse::<usize>().unwrap_or(DEFAULT_WORKERS),
        Err(_) => {
            warn!("Environment variable {} not set, using default {}.", WORKERS, DEFAULT_WORKERS);
            DEFAULT_WORKERS
        },
    }
}

pub fn get_server_addr() -> String {
    match env::var(SERVER_ADDR) {
        Ok(v) => v,

        Err(_) => {
            warn!("Environment variable {} not set, using default: {}.", SERVER_ADDR, DEFAULT_ADDR);
            DEFAULT_ADDR.to_string()
        },
    }
}

pub fn get_server_port() -> u16 {
    match env::var(SERVER_PORT) {
        Ok(v) => v.parse::<u16>().unwrap_or(DEFAULT_PORT),

        Err(_) => {
            warn!("Environment variable {} not set, using default {}.", SERVER_PORT, DEFAULT_PORT);
            DEFAULT_PORT
        },
    }
}
