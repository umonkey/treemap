//! Provides access to file-based and environment-based secrets.
//!
//! When adding or removing secrets, please update `docs/Configuration.md`

use crate::config::Config;
use crate::services::{Locatable, Locator};
use crate::types::Result;
use log::{debug, warn};
use std::fs;
use std::io::Read;
use std::sync::Arc;

#[allow(unused)]
pub struct Secrets {
    // S3 Bucket access.
    pub files_key: Option<String>,
    pub files_secret: Option<String>,

    // OSM API Access.
    pub osm_client_secret: Option<String>,
    pub osm_token: Option<String>,

    // Cloud SQLite.
    pub turso_token: Option<String>,

    // Local authentication.
    pub jwt_secret: Option<String>,

    // Background job queue.
    pub sqs_key: Option<String>,
    pub sqs_secret: Option<String>,
}

impl Secrets {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let path = &config.secrets_path.clone();

        Ok(Self {
            files_key: Self::get(path, "FILES_KEY"),
            files_secret: Self::get(path, "FILES_SECRET"),
            osm_client_secret: Self::get(path, "OSM_CLIENT_SECRET"),
            osm_token: Self::get(path, "OSM_TOKEN"),
            turso_token: Self::get(path, "TURSO_TOKEN"),
            jwt_secret: Self::get(path, "JWT_SECRET"),
            sqs_key: Self::get(path, "SQS_KEY"),
            sqs_secret: Self::get(path, "SQS_SECRET"),
        })
    }

    pub fn get(path: &str, key: &str) -> Option<String> {
        if let Ok(value) = std::env::var(key) {
            return Some(value);
        }

        let file_path = format!("{path}/{key}");

        let mut file = match fs::File::open(&file_path) {
            Ok(v) => v,

            Err(e) => {
                warn!("Error reading secret from {file_path}: {e}");
                return None;
            }
        };

        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Ok(_value) => {
                debug!("Read secret from {path}");
                Some(contents)
            }

            Err(e) => {
                warn!("Error reading {path}: {e}");
                None
            }
        }
    }
}

impl Locatable for Secrets {
    fn create(locator: &Locator) -> Result<Self> {
        let config = locator.get::<Config>()?;
        Self::new(config)
    }
}
