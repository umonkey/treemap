//! Provides access to file-based secrest.

use crate::config::Config;
use crate::services::{Locatable, Locator};
use crate::types::{Error, Result};
use log::{debug, warn};
use std::fs;
use std::io::Read;
use std::sync::Arc;

#[allow(unused)]
pub struct Secrets {
    path: String,
}

impl Secrets {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        Ok(Self {
            path: config.secrets_path.clone(),
        })
    }

    #[allow(unused)]
    pub fn get(&self, name: &str) -> Option<String> {
        let path = format!("{}/{}", self.path, name);

        let mut file = match fs::File::open(path.clone()) {
            Ok(v) => v,

            Err(e) => {
                warn!("Error opening {}: {}", path, e);
                return None;
            }
        };

        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Ok(value) => {
                debug!("Read secret from {}", path);
                Some(contents)
            }

            Err(e) => {
                warn!("Error reading {}: {}", path, e);
                None
            }
        }
    }

    #[allow(unused)]
    pub fn get_default(&self, name: &str, value: &str) -> String {
        match self.get(name) {
            Some(value) => value,
            None => value.to_string(),
        }
    }

    pub fn require(&self, name: &str) -> Result<String> {
        match self.get(name) {
            Some(value) => Ok(value),
            None => Err(Error::EnvNotSet),
        }
    }
}

impl Locatable for Secrets {
    fn create(locator: &Locator) -> Result<Self> {
        let config = locator.get::<Config>()?;
        Self::new(config)
    }
}
