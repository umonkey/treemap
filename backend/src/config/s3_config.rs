/**
 * Extract settings for the S3 client from the environment.
 *
 * Read more: https://github.com/umonkey/treemap/wiki/Configuration#s3
 */

use std::env;
use std::env::VarError;
use log::warn;

use crate::types::{Error, Result};

pub struct S3Config {
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
}

impl S3Config {
    pub fn from_env() -> Result<Self> {
        Self::get_string("AWS_ACCESS_KEY_ID")?;
        Self::get_string("AWS_SECRET_ACCESS_KEY")?;

        Ok(Self {
            bucket: Self::get_string("S3_BUCKET")?,
            region: Self::get_string("S3_REGION")?,
            endpoint: Self::get_string("S3_ENDPOINT")?,
        })
    }

    fn get_string(key: &str) -> Result<String> {
        match env::var(key) {
            Ok(v) => Ok(v),

            Err(VarError::NotPresent) => {
                warn!("Environment variable {} not set, unable to use S3. Read more at <https://github.com/umonkey/treemap/wiki/Configuration#s3>", key);
                Err(Error::EnvNotSet)
            },

            Err(VarError::NotUnicode(_)) => {
                warn!("Environment variable {} has incorrect value, unable to use S3.", key);
                Err(Error::EnvNotSet)
            },
        }
    }
}
