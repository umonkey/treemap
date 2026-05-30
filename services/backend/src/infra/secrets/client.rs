//! Provides access to file-based and environment-based secrets.
//!
//! When adding or removing secrets, please update `docs/Configuration.md`

use crate::infra::config::Config;
use crate::services::{Context, Injectable};
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

    // Mapillary API.
    pub mapillary_token: Option<String>,
    pub mapillary_org_id: Option<String>,
}

impl Secrets {
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let path = &config.secrets_path.clone();

        debug!("Looking for secrets in {path}");

        Ok(Self {
            files_key: Self::get(path, "FILES_KEY"),
            files_secret: Self::get(path, "FILES_SECRET"),
            osm_client_secret: Self::get(path, "OSM_CLIENT_SECRET"),
            osm_token: Self::get(path, "OSM_TOKEN"),
            turso_token: Self::get(path, "TURSO_TOKEN"),
            jwt_secret: Self::get(path, "JWT_SECRET"),
            sqs_key: Self::get(path, "SQS_KEY"),
            sqs_secret: Self::get(path, "SQS_SECRET"),
            mapillary_token: Self::get(path, "MAPILLARY_TOKEN"),
            mapillary_org_id: Self::get(path, "MAPILLARY_ORG_ID"),
        })
    }

    pub fn get(path: &str, key: &str) -> Option<String> {
        Self::get_env(key)
            .or_else(|| Self::get_file(path, key))
            .or_else(|| Self::get_file_lowercase(path, key))
            .or_else(|| {
                warn!("Secret {key} not found anywhere.");
                None
            })
    }

    fn get_env(key: &str) -> Option<String> {
        if let Ok(value) = std::env::var(key) {
            debug!("Secret {key} read from environment.");
            return Some(value);
        }

        None
    }

    fn get_file(path: &str, key: &str) -> Option<String> {
        Self::read_file(path, key)
    }

    fn get_file_lowercase(path: &str, key: &str) -> Option<String> {
        Self::read_file(path, &key.to_lowercase())
    }

    fn read_file(path: &str, filename: &str) -> Option<String> {
        let file_path = format!("{path}/{filename}");

        let mut file = match fs::File::open(&file_path) {
            Ok(v) => v,
            Err(_) => return None,
        };

        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Ok(_) => {
                debug!("Read secret {filename} from {path}");
                Some(contents.trim().to_string())
            }

            Err(e) => {
                warn!("Error reading secret {filename} from {path}: {e}");
                None
            }
        }
    }
}

impl Injectable for Secrets {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        let config = ctx.config();
        Self::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_get_env() {
        let key = "TEST_SECRET_ENV";
        let value = "env_value";
        std::env::set_var(key, value);
        assert_eq!(Secrets::get_env(key), Some(value.to_string()));
        std::env::remove_var(key);
    }

    #[test]
    fn test_get_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_str().unwrap();
        let key = "TEST_SECRET_FILE";
        let value = "file_value";

        let file_path = dir.path().join(key);
        let mut file = fs::File::create(file_path).unwrap();
        file.write_all(format!("  {} \n\n", value).as_bytes())
            .unwrap();

        assert_eq!(Secrets::get_file(path, key), Some(value.to_string()));
    }

    #[test]
    fn test_get_file_lowercase() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_str().unwrap();
        let key = "TEST_SECRET_LOWER";
        let value = "lower_value";

        let file_path = dir.path().join(key.to_lowercase());
        let mut file = fs::File::create(file_path).unwrap();
        file.write_all(value.as_bytes()).unwrap();

        assert_eq!(
            Secrets::get_file_lowercase(path, key),
            Some(value.to_string())
        );
    }

    #[test]
    fn test_get_chain() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_str().unwrap();

        // 1. Not found
        assert_eq!(Secrets::get(path, "MISSING"), None);

        // 2. Found in environment
        let key_env = "TEST_CHAIN_ENV";
        std::env::set_var(key_env, "env");
        assert_eq!(Secrets::get(path, key_env), Some("env".to_string()));

        // 3. Found in file (exact)
        let key_file = "TEST_CHAIN_FILE";
        let file_path = dir.path().join(key_file);
        fs::write(file_path, " file\n").unwrap();
        assert_eq!(Secrets::get(path, key_file), Some("file".to_string()));

        // 4. Found in file (lowercase)
        let key_lower = "TEST_CHAIN_LOWER";
        let file_path_lower = dir.path().join(key_lower.to_lowercase());
        fs::write(file_path_lower, "lower").unwrap();
        assert_eq!(Secrets::get(path, key_lower), Some("lower".to_string()));

        std::env::remove_var(key_env);
    }
}
