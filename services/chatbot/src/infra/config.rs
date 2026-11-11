use crate::infra::secrets::Secrets;
use anyhow::Context;
use std::env;

pub struct Config {
    pub bot_token: String,
    pub database_path: String,
    pub files_region: String,
    pub files_endpoint: String,
    pub files_bucket: String,
    pub files_key: String,
    pub files_secret: String,
    pub files_base_url: String,
    pub website_url: String,
    pub report_recipients: Vec<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let secrets_path = env::var("SECRETS_PATH").unwrap_or_else(|_| "/run/secrets".to_string());
        let secrets = Secrets::new(&secrets_path)?;

        let report_recipients = secrets
            .report_recipients
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        Ok(Self {
            bot_token: secrets.bot_token,
            database_path: env::var("CHATBOT_DATABASE").context("CHATBOT_DATABASE must be set")?,
            files_region: env::var("FILES_REGION").context("FILES_REGION must be set")?,
            files_endpoint: env::var("FILES_ENDPOINT").context("FILES_ENDPOINT must be set")?,
            files_bucket: env::var("FILES_BUCKET").context("FILES_BUCKET must be set")?,
            files_key: secrets.files_key,
            files_secret: secrets.files_secret,
            files_base_url: env::var("FILES_BASE_URL").context("FILES_BASE_URL must be set")?,
            website_url: env::var("WEBSITE_URL")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
            report_recipients,
        })
    }
}
