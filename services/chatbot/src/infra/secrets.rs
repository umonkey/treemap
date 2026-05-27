//! Provides access to file-based and environment-based secrets.

use log::{debug, warn};
use std::fs;
use std::io::Read;

pub struct Secrets {
    pub bot_token: String,
    pub files_key: String,
    pub files_secret: String,
}

impl Secrets {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        Ok(Self {
            bot_token: Self::get(path, "CHATBOT_TOKEN")?,
            files_key: Self::get(path, "FILES_KEY")?,
            files_secret: Self::get(path, "FILES_SECRET")?,
        })
    }

    pub fn get(path: &str, key: &str) -> anyhow::Result<String> {
        Self::get_env(key)
            .or_else(|| Self::get_file(path, key))
            .or_else(|| Self::get_file_lowercase(path, key))
            .ok_or_else(|| {
                warn!("Secret {key} not found anywhere.");
                anyhow::anyhow!("{key} must be set (env or file)")
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
