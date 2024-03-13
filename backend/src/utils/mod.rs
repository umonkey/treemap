use log::warn;
use std::env;

use crate::Result;
use crate::errors::Error;

pub fn getenv_usize(name: &str, default: usize) -> usize {
    match env::var(name) {
        Ok(v) => v.parse::<usize>().unwrap_or(default),
        Err(_) => {
            warn!("Environment variable {} not set, using default {}.", name, default);
            default
        },
    }
}

pub fn getenv_u16(name: &str, default: u16) -> u16 {
    match env::var(name) {
        Ok(v) => v.parse::<u16>().unwrap_or(default),
        Err(_) => {
            warn!("Environment variable {} not set, using default {}.", name, default);
            default
        },
    }
}

pub fn getenv_string(name: &str) -> Result<String> {
    match env::var(name) {
        Ok(v) => Ok(v),
        Err(_) => {
            warn!("Environment variable {} not set.", name);
            Err(Error::EnvNotSet)
        },
    }
}
